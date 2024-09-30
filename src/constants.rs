use std::{sync::Arc};
use ethers::types::{Address, H256};
use ethers::prelude::*;
use once_cell::sync::Lazy;
use ethers::providers::{Http, Provider};

// Event signatures as `Lazy` static variables
pub static ERC_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
    ))
});

pub static ERC_APPROVAL_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925"
    ))
});

pub static ERC777_SENT_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "06b541ddaa720db2b10a4d0cdac39b8d360425fc073085fac19bc82614677987"
    ))
});

pub static ERC777_MINTED_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "41cf160a08a692dd6dbfe327c2ad00486cb4bb98987f7602c0bccd146a84210a"
    ))
});

pub static ERC777_BURNED_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "a78a9be3a7b862d26933ad85fb11d80ef66b8f972d7cbba06621d583943a4098"
    ))
});

pub static ERC777_AUTHORIZED_OPERATOR_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "f4caeb2d6ca8932a215a353d0703c326ec2d81fc68170f320eb2ab49e9df61f9"
    ))
});

pub static ERC777_REVOKED_OPERATOR_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "50546e66e5f44d728365dc3908c63bc5cfeeab470722c1677e3073a6ac294aa1"
    ))
});

pub static ERC1155_SINGLE_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "c3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62"
    ))
});

pub static ERC1155_BATCH_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb"
    ))
});

pub static ERC_APPROVAL_FOR_ALL_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31"
    ))
});

// Use `Abigen` to generate the contract bindings at compile time
abigen!(
    ERC20,
    "src/abis/erc20_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    ERC721,
    "src/abis/erc721_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    ERC777,
    "src/abis/erc777_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    ERC1155,
    "src/abis/erc1155_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// Function to create an ERC20 contract instance
pub fn create_erc20_contract(
    token_address_value: &[u8],
    provider: Arc<Provider<Http>>,
) -> Result<ERC20<Provider<Http>>, Box<dyn std::error::Error + Send + Sync>> {
    let address = Address::from_slice(token_address_value);
    Ok(ERC20::new(address, provider))
}

pub fn create_erc721_contract(
    token_address_value: &[u8],
    provider: Arc<Provider<Http>>,
) -> Result<ERC721<Provider<Http>>, Box<dyn std::error::Error + Send + Sync>> {
    let address = Address::from_slice(token_address_value);
    Ok(ERC721::new(address, provider))
}

/// Function to create an ERC777 contract instance
pub fn create_erc777_contract(
    token_address_value: &[u8],
    provider: Arc<Provider<Http>>,
) -> Result<ERC777<Provider<Http>>, Box<dyn std::error::Error + Send + Sync>> {
    let address = Address::from_slice(token_address_value);
    Ok(ERC777::new(address, provider))
}

/// Function to create an ERC1155 contract instance
pub fn create_erc1155_contract(
    token_address_value: &[u8],
    provider: Arc<Provider<Http>>,
) -> Result<ERC1155<Provider<Http>>, Box<dyn std::error::Error + Send + Sync>> {
    let address = Address::from_slice(token_address_value);
    Ok(ERC1155::new(address, provider))
}