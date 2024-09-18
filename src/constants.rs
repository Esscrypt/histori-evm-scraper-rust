
use std::{fs, sync::Arc};

use ethers::{abi::Abi, types::H256};
use once_cell::sync::Lazy;
use serde_json::from_str;

pub static ERC_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f76fc01d731e"
    ))
});

pub static ERC_APPROVAL_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "8c5be1e5ebec7d5bd14f714fa2e1c5bb2eeb4c6ec2f2395f6bacc5a6d59d3f1e"
    ))
});

pub static ERC777_SENT_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "9bd9bbc6dd5b5946f3f3e462ba9e0cc1e8acff9b4c7ef6c807d94dc1c93b4f24"
    ))
});

pub static ERC777_MINTED_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "5f5f9f52bb8b9f73bca02dc3de671bb2a2c7e5d6dd2781c9fb6b4e4024728f83"
    ))
});

pub static ERC777_BURNED_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "9b7e09b62718f769df6d5dc3a85b696748f3a5c327b7d14f742d0b5190ebd722"
    ))
});

pub static ERC777_AUTHORIZED_OPERATOR_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "173d780bf85f0da20d848506f4347c3a7e5dbf774f41648da6f580a948fbb8d1"
    ))
});

pub static ERC777_REVOKED_OPERATOR_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "af9845cc27f9c57597a07b431f3e4e9b22b1a560f55ad22a4dbf2685cdb57d4a"
    ))
});

pub static ERC1155_SINGLE_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "c3d58168c5c3a4a9f2df6b62ce6d5e1f10be9e329b27e58b9f3a7ac2e62a63f4"
    ))
});

pub static ERC1155_BATCH_TRANSFER_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "4a39dc06d4c0dbc64b70b41c855c3a75bca68750fbb8ca7bff2d9c78d0d3c7e8"
    ))
});

pub static ERC_APPROVAL_FOR_ALL_SIGNATURE: Lazy<H256> = Lazy::new(|| {
    H256::from_slice(&hex_literal::hex!(
        "8c5be1e5ebec7d5bd14f714fa2e1c5bb2eeb4c6ec2f2395f6bacc5a6d59d3f1e"
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
