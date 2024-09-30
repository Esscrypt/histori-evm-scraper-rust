
use std::{fs, sync::Arc};

use ethers::{abi::Abi, types::H256};
use once_cell::sync::Lazy;
use serde_json::from_str;
use ethers::utils::keccak256;

// Dynamically generate the hash for the Transfer event signature
pub static ERC_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("Transfer(address,address,uint256)"))
});

pub static ERC_APPROVAL_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("Approval(address,address,uint256)"))
});

pub static ERC777_SENT_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("Sent(address,address,address,uint256,bytes,bytes)"))
});

pub static ERC777_MINTED_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("Minted(address,address,uint256,bytes,bytes)"))
});

pub static ERC777_BURNED_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("Burned(address,address,uint256,bytes,bytes)"))
});

pub static ERC777_AUTHORIZED_OPERATOR_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("AuthorizedOperator(address,address)"))
});

pub static ERC777_REVOKED_OPERATOR_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("RevokedOperator(address,address)"))
});

pub static ERC1155_SINGLE_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("TransferSingle(address,address,address,uint256,uint256)"))
});

pub static ERC1155_BATCH_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("TransferBatch(address,address,address,uint256[],uint256[])"))
});

pub static ERC_APPROVAL_FOR_ALL_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from(keccak256("ApprovalForAll(address,address,bool)"))
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
