// use diesel::prelude::*;
use std::sync::Arc;
use crate::models::{NewToken, NewTokenID, Token, TokenID};
use crate::{create_erc1155_contract, create_erc20_contract, create_erc721_contract, create_erc777_contract, TokenType};
use ethers::providers::{Provider, Http};

use diesel::prelude::*;
use diesel::insert_into;
use crate::schema::tokens::dsl::*;
use crate::PgPooledConnection;

use std::convert::TryInto;

pub async fn check_and_insert_token(
    conn: &mut PgPooledConnection,
    provider: Arc<Provider<Http>>,
    token_address_value: &[u8],
    current_block_number: i32,
    erc_type: TokenType,
) -> Result<Token, Box<dyn std::error::Error + Send + Sync>> {
    // Check if the token exists in the database
    let token_exists = tokens
        .filter(token_address.eq(token_address_value))
        .first::<Token>(conn)
        .optional()?;

    if let Some(existing_token) = token_exists {
        return Ok(existing_token); // Return the existing token
    }

    // Fetch metadata based on the token type
    let (erc_name, erc_symbol, erc_decimals, erc_granularity) = match erc_type {
        TokenType::ERC20 => {
            let contract = create_erc20_contract(token_address_value, provider.clone())?;
            let erc_name: String = contract.name().call().await?;
            let erc_symbol: String = contract.symbol().call().await?;
            let erc_decimals: i16 = contract.decimals().call().await?.try_into().map_err(|_| "Failed to convert decimals to i16")?;
            (erc_name, erc_symbol, Some(erc_decimals), None)
        }
        TokenType::ERC721 => {
            let contract = create_erc721_contract(token_address_value, provider.clone())?;
            let erc_name: String = contract.name().call().await.unwrap_or_default(); // Handle optional metadata gracefully
            let erc_symbol: String = contract.symbol().call().await.unwrap_or_default();
            (erc_name, erc_symbol, None, None) // No decimals for ERC721
        }
        TokenType::ERC777 => {
            let contract = create_erc777_contract(token_address_value, provider.clone())?;
            let erc_name: String = contract.name().call().await?;
            let erc_symbol: String = contract.symbol().call().await?;
            let erc_granularity: i64 = contract.granularity().call().await?.try_into().map_err(|_| "Failed to convert granularity to i64")?;
            (erc_name, erc_symbol, None, Some(erc_granularity)) // ERC777 has granularity
        }
        TokenType::ERC1155 => {
            // ERC1155 does not have `name` and `symbol` methods in the standard, return default values
            ("ERC1155 Token".to_string(), "ERC1155".to_string(), None, None)
        }
    };

    // Construct the new token
    let new_token = NewToken {
        token_address: token_address_value,
        block_number: current_block_number,
        token_type: match erc_type {
            TokenType::ERC20 => "ERC20",
            TokenType::ERC721 => "ERC721",
            TokenType::ERC777 => "ERC777",
            TokenType::ERC1155 => "ERC1155",
        },
        name: &erc_name,
        symbol: &erc_symbol,
        decimals: erc_decimals,
        granularity: erc_granularity,
    };

    // Insert the new token into the database
    insert_into(tokens)
        .values(&new_token)
        .execute(conn)?;

    Ok(Token {
        token_address: token_address_value.to_vec(),
        block_number: current_block_number,
        token_type: new_token.token_type.to_string(),
        name: erc_name,
        symbol: erc_symbol,
        decimals: erc_decimals,
        granularity: erc_granularity,
    })
}
// The rest of the code remains the same
pub async fn fetch_and_store_token_uri(
    conn: &mut PgPooledConnection,
    contract_address_value: &[u8],
    token_id_value: i16,
    erc_type: TokenType,
    provider: Arc<Provider<Http>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use crate::schema::token_ids::dsl::*;

    let existing_token_id: Option<TokenID> = token_ids
        .filter(contract_address.eq(contract_address_value))
        .filter(token_id.eq(token_id_value))
        .first::<TokenID>(conn)
        .optional()?;

    if let Some(existing_token) = existing_token_id {
        if existing_token.token_uri.is_some() {
            println!("Token metadata already exists for contract {:?}, tokenId {}", contract_address_value, token_id_value);
            return Ok(());
        }
    }

    let uri: Option<String> = match erc_type {
        TokenType::ERC721 => {
            let contract = create_erc721_contract(contract_address_value, provider.clone())?;
            Some(contract.method::<_, String>("tokenURI", token_id_value)?.call().await?)
        }
        TokenType::ERC1155 => {
            let contract = create_erc1155_contract(contract_address_value, provider.clone())?;
            Some(contract.method::<_, String>("uri", token_id_value)?.call().await?)
        }
        _ => None,
    };


    if let Some(uri) = uri {
            let new_token_id = NewTokenID {
                contract_address: contract_address_value,
                token_id: token_id_value,
                token_uri: Some(uri),
            };

            diesel::insert_into(token_ids).values(&new_token_id).execute(conn)?;
        }

        println!("Persisted tokenURI for contract {:x?}, tokenId {}", contract_address_value, token_id_value);

    Ok(())
}
