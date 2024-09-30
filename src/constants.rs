
use std::{fs, sync::Arc};

use ethers::{abi::Abi, types::H256};
use once_cell::sync::Lazy;
use serde_json::from_str;

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

// Function to load ABI from a file
fn load_abi(path: &str) -> Abi {
    let abi = fs::read_to_string(path).expect("ABI file not found");
    from_str(&abi).expect("Failed to parse ABI")
}

// Define ABIs as static variables, loaded once for the entire lifetime of the program
pub static ERC20_ABI: Lazy<Arc<Abi>> = Lazy::new(|| Arc::new(load_abi("./abis/erc20_abi.json")));
pub static ERC721_ABI: Lazy<Arc<Abi>> = Lazy::new(|| Arc::new(load_abi("./abis/erc721_abi.json")));
pub static ERC777_ABI: Lazy<Arc<Abi>> = Lazy::new(|| Arc::new(load_abi("./abis/erc777_abi.json")));
pub static ERC1155_ABI: Lazy<Arc<Abi>> = Lazy::new(|| Arc::new(load_abi("./abis/erc1155_abi.json")));
