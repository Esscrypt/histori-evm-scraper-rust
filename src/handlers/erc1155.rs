use std::sync::Arc;

use ethers::providers::{Http, Provider};
use ethers::types::{Address, Log, H256, U256};

use crate::models::balance::update_historical_balance;
use crate::models::allowance::update_historical_allowance;
use crate::token_service::{check_and_insert_token, fetch_and_store_token_uri};
use crate::{Cli, PgPooledConnection, TokenType, ERC1155_BATCH_TRANSFER_SIGNATURE, ERC1155_SINGLE_TRANSFER_SIGNATURE, ERC_APPROVAL_FOR_ALL_SIGNATURE};

pub async fn handle_erc1155_event(log: &Log, conn: &mut PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let event_signature: H256 = log.topics[0];

    // ERC1155 Event Signatures
    let transfer_single_event_signature = *ERC1155_SINGLE_TRANSFER_SIGNATURE;  // ERC1155 TransferSingle event signature
    let transfer_batch_event_signature = *ERC1155_BATCH_TRANSFER_SIGNATURE;  // ERC1155 TransferBatch event signature
    let approval_for_all_event_signature = *ERC_APPROVAL_FOR_ALL_SIGNATURE;  // ERC1155 ApprovalForAll event signature

    match event_signature {
        sig if sig == transfer_single_event_signature => handle_erc1155_transfer_single(log, conn, provider, cli).await?,
        sig if sig == transfer_batch_event_signature => handle_erc1155_transfer_batch(log, conn, provider, cli).await?,
        sig if sig == approval_for_all_event_signature => handle_erc1155_approval_for_all(log, conn, cli).await?,
        _ => println!("Unknown ERC1155 event at address: {:?}", log.address),
    }

    Ok(())
}

async fn handle_erc1155_transfer_single(log: &Log, conn: &mut PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse TransferSingle event
    let _operator = Address::from(log.topics[1]);
    let from = Address::from(log.topics[2]);
    let to = Address::from(log.topics[3]);
    let (token_id, value) = parse_transfer_single(log.data.to_vec());  // Assumes a `parse_transfer_single` helper function
    let block_number = log.block_number.unwrap().as_u32() as i32;
    
    // Pass the token type (ERC1155 in this case) to check_and_insert_token
    check_and_insert_token(conn, provider.clone(), log.address.as_bytes(), block_number, TokenType::ERC1155).await?;
    if cli.process_token_uri {
        fetch_and_store_token_uri(conn, log.address.as_bytes(), token_id, TokenType::ERC721, provider.clone()).await?;
    }


    if cli.process_balances {
        // Update the balance for the sender (subtract)
        update_historical_balance(conn, from.as_bytes(), log.address.as_bytes(), -(value as i64), Some(token_id as i16), "ERC1155", log.block_number.unwrap().as_u32() as i32).unwrap();

        // Update the balance for the recipient (add)
        update_historical_balance(conn, to.as_bytes(), log.address.as_bytes(), value as i64, Some(token_id as i16), "ERC1155", log.block_number.unwrap().as_u32() as i32).unwrap();
    }

    Ok(())
}

async fn handle_erc1155_transfer_batch(log: &Log, conn: &mut PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse TransferBatch event
    let _operator = Address::from(log.topics[1]);
    let from = Address::from(log.topics[2]);
    let to = Address::from(log.topics[3]);
    let block_number = log.block_number.unwrap().as_u32() as i32;
    
    let (token_ids, values) = parse_transfer_batch(log.data.0.to_vec());  // Assumes a `parse_transfer_batch` helper function

        // Update the balance for each token_id in the batch
        for (token_id, value) in token_ids.iter().zip(values.iter()) {
            // Pass the token type (ERC1155 in this case) to check_and_insert_token
            check_and_insert_token(conn, provider.clone(), log.address.as_bytes(), block_number, TokenType::ERC1155).await?;
    
            if cli.process_token_uri {
                fetch_and_store_token_uri(conn, log.address.as_bytes(), *token_id, TokenType::ERC721, provider.clone()).await?;
            }
            if cli.process_balances {
                // Update the balance for the sender (subtract)
                update_historical_balance(conn, from.as_bytes(), log.address.as_bytes(), -(*value as i64), Some(*token_id as i16), "ERC1155", log.block_number.unwrap().as_u32() as i32).unwrap();

                // Update the balance for the recipient (add)
                update_historical_balance(conn, to.as_bytes(), log.address.as_bytes(), *value as i64, Some(*token_id as i16), "ERC1155", log.block_number.unwrap().as_u32() as i32).unwrap();
            }
        }


    Ok(())
}

async fn handle_erc1155_approval_for_all(log: &Log, conn: &mut PgPooledConnection, cli: &Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !cli.process_allowances { return Ok(()) }
    // Parse ApprovalForAll event
    let owner = Address::from(log.topics[1]);
    let operator = Address::from(log.topics[2]);
    let approved = log.data.0[31] != 0;  // Approval flag in the event data

    let value = if approved { 1 } else { 0 };  // Set allowance to 1 for approval, 0 for revocation

    // Update operator allowance for all tokens owned by the user (without token_id)
    update_historical_allowance(conn, owner.as_bytes(), operator.as_bytes(), log.address.as_bytes(), value, None, "ERC1155", log.block_number.unwrap().as_u32() as i32).unwrap();

    Ok(())
}

fn parse_transfer_single(data: Vec<u8>) -> (i16, u64) {
    let token_id = U256::from_big_endian(&data[0..32]).as_u32() as i16;
    let value = U256::from_big_endian(&data[32..64]).as_u64();
    (token_id, value)
}

fn parse_transfer_batch(data: Vec<u8>) -> (Vec<i16>, Vec<u64>) {
    let token_count = data.len() / 64;  // Each token ID and value takes 32 bytes each
    let mut token_ids = Vec::new();
    let mut values = Vec::new();

    for i in 0..token_count {
        let token_id = U256::from_big_endian(&data[i * 32..(i + 1) * 32]).as_u32() as i16;
        let value = U256::from_big_endian(&data[(i + token_count) * 32..(i + token_count + 1) * 32]).as_u64();
        token_ids.push(token_id);
        values.push(value);
    }

    (token_ids, values)
}
