use std::sync::Arc;

use ethers::providers::{Http, Provider};
use ethers::types::{Address, Log, H256, U256};
use log::info;

use crate::token_service::check_and_insert_token;
use crate::{Cli, PgPooledConnection, TokenType, ERC_APPROVAL_SIGNATURE, ERC_TRANSFER_SIGNATURE};

use crate::models::allowance::update_historical_allowance;
use crate::models::balance::update_historical_balance;
use crate::models::token_supply::update_total_supply;

pub async fn handle_erc20_event(log: &Log, conn: &mut PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Arc<Cli>) ->  Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    let event_signature: H256 = log.topics[0];

    // ERC20 Event Signatures
    let transfer_event_signature: H256 = *ERC_TRANSFER_SIGNATURE; // ERC20 Transfer event signature
    let approval_event_signature: H256 = *ERC_APPROVAL_SIGNATURE; // ERC20 Approval event signature

    match event_signature {
        sig if sig == transfer_event_signature => handle_erc20_log(log, conn, provider, cli).await?,
        sig if sig == approval_event_signature => {
            handle_erc20_allowance(log, conn, cli).await?;
        }
        _ => println!("Unknown ERC20 event at address: {:?}", log.address),
    }

    Ok(())
}

async fn handle_erc20_log(log: &Log, conn: &mut PgPooledConnection, provider: Arc<Provider<Http>>, cli: &Arc<Cli>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    info!(
        "Handling ERC20 transfer log for token address: {:?}",
        log.address
    );

    let from = Address::from(log.topics[1]);
    let to = Address::from(log.topics[2]);
    let value = U256::from_big_endian(&log.data.0).as_u64() as i64;
    let block_number = log.block_number.unwrap().as_u32() as i32;

    // Pass the token type (ERC20 in this case) to check_and_insert_token
    check_and_insert_token(conn, provider, log.address.as_bytes(), block_number, TokenType::ERC20).await?;


    if cli.process_balances {
        info!(
            "Updating balance for sender: {:?}, recipient: {:?}",
            from, to
        );
        update_historical_balance(
            conn,
            from.as_bytes(),
            log.address.as_bytes(),
            -value,
            None,
            "ERC20",
            block_number,
        )
        .unwrap();
        update_historical_balance(
            conn,
            to.as_bytes(),
            log.address.as_bytes(),
            value,
            None,
            "ERC20",
            block_number,
        )
        .unwrap();
    }
    if cli.process_total_supplies {
        // Handle minting or burning
        let zero_address = Address::zero();
        if from == zero_address {
            info!("Minting detected for token address: {:?}", log.address);
            update_total_supply(conn, log.address.as_bytes(), value, block_number)?;
        } else if to == zero_address {
            info!("Burning detected for token address: {:?}", log.address);
            update_total_supply(conn, log.address.as_bytes(), -value, block_number)?;
        }
    }

    Ok(())
}

async fn handle_erc20_allowance(log: &Log, conn: &mut PgPooledConnection, cli: &Arc<Cli>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if cli.process_allowances {
        // Parse Approval event
        let owner = Address::from(log.topics[1]);
        let spender = Address::from(log.topics[2]);
        let value = U256::from_big_endian(&log.data.0).as_u64() as i64;

        // Update the allowance with historical tracking (adding or subtracting the value)
        update_historical_allowance(
            conn,
            owner.as_bytes(),
            spender.as_bytes(),
            log.address.as_bytes(),
            value,
            None,
            "ERC20",
            log.block_number.unwrap().as_u32() as i32,
        )?;
    }

    Ok(())
}
