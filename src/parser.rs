use std::sync::Arc;

use ethers::providers::{Http, Provider};
use ethers::types::{Log, H160};
use crate::utils::determine_token_type;
use crate::handlers::{handle_erc20_event, handle_erc721_event, handle_erc1155_event, handle_erc777_event};
use crate::{Cli, PgPooledConnection};

/// Function to parse ERC20, ERC721, ERC1155, and ERC777 log
/// This function checks the token type and dispatches the appropriate handler.
pub async fn parse_log(
    log: &Log, 
    conn: &mut PgPooledConnection, 
    provider: Arc<Provider<Http>>, 
    cli: &Arc<Cli>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token_address: H160 = log.address;

    // Determine the token type based on the address
    let token_type = determine_token_type(provider.clone(), token_address).await;

    // Dispatch the log to the appropriate handler based on token type
    match token_type.as_str() {
        "ERC20" => handle_erc20_event(log, conn, provider, cli).await?,
        "ERC721" => handle_erc721_event(log, conn, provider, cli).await?,
        "ERC1155" => handle_erc1155_event(log, conn, provider, cli).await?,
        "ERC777" => handle_erc777_event(log, conn, provider, cli).await?,
        _ => println!("Unknown token type at address: {:?}", token_address),
    }

    Ok(())
}
