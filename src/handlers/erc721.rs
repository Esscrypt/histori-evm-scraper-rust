use std::sync::Arc;

use ethers::providers::{Http, Provider};
use ethers::types::{Address, Log, H256, U256};

use crate::models::balance::update_historical_balance;
use crate::models::allowance::update_historical_allowance;
use crate::token_service::{check_and_insert_token, fetch_and_store_token_uri};
use crate::{Cli, PgPooledConnection, TokenType, ERC_APPROVAL_FOR_ALL_SIGNATURE, ERC_APPROVAL_SIGNATURE, ERC_TRANSFER_SIGNATURE};


pub async fn handle_erc721_event(log: &Log, conn: &mut PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Arc<Cli>) -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> {
    let event_signature: H256 = log.topics[0];

    // ERC721 Event Signatures
    let transfer_event_signature: H256 = *ERC_TRANSFER_SIGNATURE;  // ERC721 Transfer event signature
    let approval_event_signature: H256 = *ERC_APPROVAL_SIGNATURE;  // ERC721 Approval event signature
    let approval_for_all_event_signature: H256 = *ERC_APPROVAL_FOR_ALL_SIGNATURE;  // ERC721 ApprovalForAll event signature

    match event_signature {
        sig if sig == transfer_event_signature => handle_erc721_log(log, conn, provider, cli).await?,
        sig if sig == approval_event_signature => handle_erc721_allowance(log, conn, cli).await?,
        sig if sig == approval_for_all_event_signature => handle_erc721_approval_for_all(log, conn, cli).await?,
        _ => println!("Unknown ERC721 event at address: {:?}", log.address),
    }

    Ok(())
}


async fn handle_erc721_log(log: &Log, conn: &mut PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Arc<Cli>) -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> {
    let block_number = log.block_number.unwrap().as_u32() as i32;

    let from = Address::from(log.topics[1]);
    let to = Address::from(log.topics[2]);
    let token_id: i16 = U256::from(log.topics[3].0).as_u64() as i16;

    // Pass the token type (ERC20 in this case) to check_and_insert_token
    check_and_insert_token(conn, provider.clone(), log.address.as_bytes(), block_number, TokenType::ERC721).await?;
    
    if cli.process_token_uri {
        fetch_and_store_token_uri(conn, log.address.as_bytes(), token_id, TokenType::ERC721, provider.clone()).await?;
    }

    if cli.process_balances {
        // Parse Transfer event

        // Update the balance for the sender (subtract ownership) with historical tracking
        update_historical_balance(conn, from.as_bytes(), log.address.as_bytes(), -1, Some(token_id), "ERC721", log.block_number.unwrap().as_u64() as i32)?;

        // Update the balance for the recipient (add ownership) with historical tracking
        update_historical_balance(conn, to.as_bytes(), log.address.as_bytes(), 1, Some(token_id), "ERC721", log.block_number.unwrap().as_u64() as i32)?;
    }

    Ok(())
}

async fn handle_erc721_allowance(log: &Log, conn: &mut PgPooledConnection, cli: &Cli) -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> {
    if cli.process_allowances {
        // Parse Approval event
        let owner = Address::from(log.topics[1]);
        let approved = Address::from(log.topics[2]);
        let token_id = U256::from(log.topics[3].0).as_u32() as i16;

        // Insert approval for a specific token_id with historical tracking
        update_historical_allowance(conn, owner.as_bytes(), approved.as_bytes(), log.address.as_bytes(), 1, Some(token_id), "ERC721", log.block_number.unwrap().as_u64() as i32).unwrap();
    }

    Ok(())
}

async fn handle_erc721_approval_for_all(log: &Log, conn: &mut PgPooledConnection, cli: &Arc<Cli>) -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> {
    if cli.process_allowances {
        // Parse ApprovalForAll event
        let owner = Address::from(log.topics[1]);
        let operator = Address::from(log.topics[2]);
        let approved: bool = log.data.0[31] != 0;

        let value = if approved { 1 } else { 0 };  // Set allowance to 1 for approval, 0 for revocation

        // Update operator allowance for all tokens owned by the user (without token_id)
        update_historical_allowance(conn, owner.as_bytes(), operator.as_bytes(), log.address.as_bytes(), value, None, "ERC721", log.block_number.unwrap().as_u64() as i32).unwrap();
    }

    Ok(())
}
