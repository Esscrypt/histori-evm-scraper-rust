use std::sync::Arc;
use std::{fs, io::{self, Write}};
// use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};

// Import contract instances using abigen methods from `constants.rs`
use crate::constants::{create_erc20_contract, create_erc721_contract, create_erc1155_contract, create_erc777_contract};

// Check if a token implements the ERC20 standard by querying the `decimals()` method
async fn is_erc20(provider: Arc<Provider<Http>>, token_address: Address) -> bool {
    match create_erc20_contract(&token_address.as_bytes(), provider.clone()) {
        Ok(contract) => {
            match contract.method::<(), u8>("decimals", ()).unwrap().call().await {
                Ok(_) => true,  // Decimals method is supported, it's an ERC20 token
                Err(e) => {
                    eprintln!("Error calling decimals() on ERC20: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create ERC20 contract instance: {:?}", e);
            false
        }
    }
}

// Check if a token implements the ERC721 standard by querying `supportsInterface(0x80ac58cd)`
async fn is_erc721(provider: Arc<Provider<Http>>, token_address: Address) -> bool {
    match create_erc721_contract(&token_address.as_bytes(), provider.clone()) {
        Ok(contract) => {
            let erc721_interface: [u8; 4] = [0x80, 0xac, 0x58, 0xcd];
            match contract.method::<[u8; 4], bool>("supportsInterface", erc721_interface).unwrap().call().await {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error calling supportsInterface() on ERC721: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create ERC721 contract instance: {:?}", e);
            false
        }
    }
}

// Check if a token implements the ERC1155 standard by querying `supportsInterface(0xd9b67a26)`
async fn is_erc1155(provider: Arc<Provider<Http>>, token_address: Address) -> bool {
    match create_erc1155_contract(&token_address.as_bytes(), provider.clone()) {
        Ok(contract) => {
            let erc1155_interface: [u8; 4] = [0xd9, 0xb6, 0x7a, 0x26];
            match contract.method::<[u8; 4], bool>("supportsInterface", erc1155_interface).unwrap().call().await {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error calling supportsInterface() on ERC1155: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create ERC1155 contract instance: {:?}", e);
            false
        }
    }
}

// Check if a token implements the ERC777 standard by querying the `granularity()` method
async fn is_erc777(provider: Arc<Provider<Http>>, token_address: Address) -> bool {
    match create_erc777_contract(&token_address.as_bytes(), provider.clone()) {
        Ok(contract) => {
            match contract.method::<(), U256>("granularity", ()).unwrap().call().await {
                Ok(_) => true,  // Granularity method is supported, it's an ERC777 token
                Err(e) => {
                    eprintln!("Error calling granularity() on ERC777: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create ERC777 contract instance: {:?}", e);
            false
        }
    }
}

// Determines the token type by querying the contract at `token_address`.
pub async fn determine_token_type(provider: Arc<Provider<Http>>, token_address: Address) -> String {
    if is_erc20(provider.clone(), token_address).await {
        "ERC20".to_string()
    } else if is_erc721(provider.clone(), token_address).await {
        "ERC721".to_string()
    } else if is_erc1155(provider.clone(), token_address).await {
        "ERC1155".to_string()
    } else if is_erc777(provider.clone(), token_address).await {
        "ERC777".to_string()
    } else {
        "Unknown".to_string()
    }
}

// Function to read the last processed block from a file
pub fn read_last_processed_block(file_path: &str) -> u64 {
    match fs::read_to_string(file_path) {
        Ok(contents) => contents.trim().parse().unwrap_or(0),  // Return 0 if the file is empty or parsing fails
        Err(_) => 0,  // If the file doesn't exist, return 0
    }
}

// Function to write the last processed block to a file
pub fn write_last_processed_block(file_path: &str, block_number: u64) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    writeln!(file, "{}", block_number)?;
    Ok(())
}