use std::sync::Arc;

use ethers::providers::{Http, Provider};
use ethers::types::{Address, Log, H256, U256};

use crate::models::balance::update_historical_balance;
use crate::models::allowance::update_historical_allowance;
use crate::models::token_supply::update_total_supply;
use crate::token_service::check_and_insert_token;
use crate::{Cli, PgPooledConnection, TokenType, ERC777_AUTHORIZED_OPERATOR_SIGNATURE, ERC777_BURNED_SIGNATURE, ERC777_MINTED_SIGNATURE, ERC777_REVOKED_OPERATOR_SIGNATURE, ERC777_SENT_SIGNATURE};


pub async fn handle_erc777_event(log: &Log, conn: &mut PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Arc<Cli>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let event_signature: H256 = log.topics[0];

    // ERC777 Event Signatures
    let sent_event_signature: H256 = *ERC777_SENT_SIGNATURE;  // ERC777 Sent event
    let minted_event_signature: H256 = *ERC777_MINTED_SIGNATURE;  // ERC777 Minted event
    let burned_event_signature: H256 = *ERC777_BURNED_SIGNATURE;  // ERC777 Burned event
    let authorized_operator_event_signature: H256 = *ERC777_AUTHORIZED_OPERATOR_SIGNATURE;  // ERC777 AuthorizedOperator
    let revoked_operator_event_signature: H256 = *ERC777_REVOKED_OPERATOR_SIGNATURE;  // ERC777 RevokedOperator

    match event_signature {
        sig if sig == sent_event_signature => handle_erc777_sent(log, conn, provider, cli).await?,
        sig if sig == minted_event_signature => handle_erc777_minted(log, conn, cli).await?,
        sig if sig == burned_event_signature => handle_erc777_burned(log, conn, cli).await?,
        sig if sig == authorized_operator_event_signature => handle_erc777_authorized_operator(log, conn, cli).await?,
        sig if sig == revoked_operator_event_signature => handle_erc777_revoked_operator(log, conn, cli).await?,
        _ => println!("Unknown ERC777 event at address: {:?}", log.address),
    }

    Ok(())
}

async fn handle_erc777_sent(log: &Log, conn: &mut  PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Arc<Cli>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse Sent event
    let _operator = Address::from(log.topics[1]);
    let from = Address::from(log.topics[2]);
    let to = Address::from(log.topics[3]);
    let value = U256::from_big_endian(&log.data[0..32]).as_u64() as i64;
    let block_number = log.block_number.unwrap().as_u32() as i32;

    // Pass the token type (ERC20 in this case) to check_and_insert_token
    check_and_insert_token(conn, provider, log.address.as_bytes(), block_number, TokenType::ERC1155).await?;

    if cli.process_balances {
        // Update the balance for the sender (subtract)
        update_historical_balance(conn, from.as_bytes(), log.address.as_bytes(), -value, None, "ERC777", log.block_number.unwrap().as_u64() as i32).unwrap();

        // Update the balance for the recipient (add)
        update_historical_balance(conn, to.as_bytes(), log.address.as_bytes(), value, None, "ERC777", log.block_number.unwrap().as_u64() as i32).unwrap();

    }

    Ok(())
}

async fn handle_erc777_minted(log: &Log, conn: &mut PgPooledConnection, cli: &Arc<Cli>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse Minted event
    let _operator = Address::from(log.topics[1]);
    let to = Address::from(log.topics[2]);
    let value = U256::from_big_endian(&log.data[0..32]).as_u64() as i64;

    if cli.process_balances {
        // Update the balance for the recipient (add)
        update_historical_balance(conn, to.as_bytes(), log.address.as_bytes(), value, None, "ERC777", log.block_number.unwrap().as_u32() as i32).unwrap();
    }
    if cli.process_total_supplies {
        // Increase the total supply
        update_total_supply(conn, log.address.as_bytes(), value, log.block_number.unwrap().as_u32() as i32)?;
    }

    Ok(())
}

async fn handle_erc777_burned(log: &Log, conn: &mut PgPooledConnection, cli: &Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse Burned event
    let _operator = Address::from(log.topics[1]);
    let from = Address::from(log.topics[2]);
    let value = U256::from_big_endian(&log.data[0..32]).as_u64() as i64;

    if cli.process_balances {
        // Update the balance for the sender (subtract)
        update_historical_balance(conn, from.as_bytes(), log.address.as_bytes(), -value, None, "ERC777", log.block_number.unwrap().as_u32() as i32).unwrap();
    }
    if cli.process_total_supplies {
        // Decrease the total supply
        update_total_supply(conn, log.address.as_bytes(), -value, log.block_number.unwrap().as_u32() as i32).unwrap();
    }

    Ok(())
}

async fn handle_erc777_authorized_operator(log: &Log, conn: &mut PgPooledConnection, cli: &Arc<Cli>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !cli.process_allowances { return Ok(()) } ;

    // Parse AuthorizedOperator event
    let holder = Address::from(log.topics[1]);
    let operator = Address::from(log.topics[2]);

    // Update allowance for the operator (1 means authorized)
    update_historical_allowance(conn, holder.as_bytes(), operator.as_bytes(), log.address.as_bytes(), 1, None, "ERC777", log.block_number.unwrap().as_u32() as i32).unwrap();

    Ok(())
}

async fn handle_erc777_revoked_operator(log: &Log, conn: &mut PgPooledConnection, cli: &Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !cli.process_allowances { return Ok(())} ;
    // Parse RevokedOperator event
    let holder = Address::from(log.topics[1]);
    let operator = Address::from(log.topics[2]);

    // Update allowance for the operator (0 means revoked)
    update_historical_allowance(conn, holder.as_bytes(), operator.as_bytes(), log.address.as_bytes(), 0, None, "ERC777", log.block_number.unwrap().as_u32() as i32).unwrap();

    Ok(())
}
