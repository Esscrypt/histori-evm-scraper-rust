mod db;
mod parser;
mod schema;
mod utils;
mod token_service;
mod models;
mod handlers;
mod constants;  // Import the constants module

use std::sync::Arc;
use std::collections::HashSet; 
use clap::Parser;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{BlockNumber, Filter, Log, H256};
use futures::future::join_all;
use log::{info, error};
use parser::parse_log;
use r2d2::PooledConnection;
use utils::{read_last_processed_block, write_last_processed_block};
use std::env;
use tokio::task::JoinHandle;
use crate::db::establish_connection_pool;
use crate::constants::*;  // Import all constants

pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

// Define supported token types as an enum for CLI options
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    ERC20,
    ERC721,
    ERC1155,
    ERC777,
}

// Define CLI arguments
#[derive(Parser)]
#[command(name = "Token Scraper CLI")]
#[command(about = "Scrape logs for selected token types", long_about = None)]
pub struct Cli {
    /// Include ERC20 events
    #[arg(long)]
    erc20: bool,

    /// Include ERC721 events
    #[arg(long)]
    erc721: bool,

    /// Include ERC1155 events
    #[arg(long)]
    erc1155: bool,

    /// Include ERC777 events
    #[arg(long)]
    erc777: bool,

    /// Process balances
    #[arg(long)]
    process_balances: bool,

    /// Process allowances
    #[arg(long)]
    process_allowances: bool,

    /// Process total supplies
    #[arg(long)]
    process_total_supplies: bool,

    #[arg(long)]
    process_token_uri: bool,
}

#[tokio::main]
async fn main() {
    // Initialize the logger
    dotenv().ok();
    env_logger::init();  // Alternatively, use flexi_logger for more features

    info!("Starting the Token Scraper CLI");

    // Set up connection pool
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = establish_connection_pool();

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let provider: Arc<Provider<Http>> = Arc::new(Provider::<Http>::try_from(rpc_url).expect("Invalid RPC URL"));

    let block_range: u64 = env::var("BLOCK_RANGE").unwrap_or_else(|_| "10000".to_string()).parse().expect("Invalid BLOCK_RANGE");

    // Parse CLI arguments and wrap in Arc for thread-safe sharing
    let cli: Arc<Cli> = Arc::new(Cli::parse());

    let mut from_block: u64 = read_last_processed_block("lastProcessedBlock.txt");


   let mut latest_finalized_block = provider.get_block_number().await.unwrap().0[0];

   info!("Starting the block processing loop");

    // Continuously process blocks
    while from_block < latest_finalized_block {
        // Calculate the block range for the current iteration
        let to_block = if from_block > latest_finalized_block - block_range { latest_finalized_block } else { from_block + block_range};  // Ensure from_block doesn't exceed current_block
        
        info!("Processing blocks from {} to {}", from_block, to_block);

        // Fetch logs based on the selected token types
        let logs = fetch_logs(&provider, from_block, to_block, &cli).await;

        // Dispatch each log to its own task using tokio::spawn
        let tasks: Vec<JoinHandle<()>> = logs
            .into_iter()
            .map(|log| {
                let pool_clone = pool.clone();   // Clone the pool
                let provider_clone = provider.clone();   // Clone the Provider instance
                let cli_clone = Arc::clone(&cli); // Clone the Arc for `Cli`

                tokio::spawn(async move {
                    let conn: &mut PgPooledConnection = &mut pool_clone.get().expect("Failed to get connection from pool");  // Get a connection from the pool
                    if let Err(e) = parse_log(&log, conn, provider_clone, &cli_clone).await {
                        error!("Error parsing log: {:?}", e);
                    }
                })
            })
            .collect();

        // Wait for all the spawned threads to finish
        join_all(tasks).await;

        info!("Finished processing blocks from {} to {}", from_block, to_block);
        let _ = write_last_processed_block("lastProcessedBlock.txt", to_block);

        // Move to the next block range
        from_block = to_block + block_range;
        latest_finalized_block = provider.get_block_number().await.unwrap().0[0];
    }
}

async fn fetch_logs(provider: &Provider<Http>, from_block: u64, to_block: u64, cli: &Arc<Cli>) -> Vec<Log> {
    let mut topics: HashSet<H256> = HashSet::new();  // Use HashSet to store unique topics

    // Add event signatures based on the CLI token type options
    if cli.erc20 || cli.erc721 {
        topics.insert(*ERC_TRANSFER_SIGNATURE);
        topics.insert(*ERC_APPROVAL_SIGNATURE);
    }

    if cli.erc1155 {
        topics.insert(*ERC1155_BATCH_TRANSFER_SIGNATURE);
        topics.insert(*ERC1155_SINGLE_TRANSFER_SIGNATURE);
        topics.insert(*ERC_APPROVAL_FOR_ALL_SIGNATURE);  // ERC1155 shares ApprovalForAll with ERC721
    }

    if cli.erc777 {
        topics.insert(*ERC777_SENT_SIGNATURE);
        topics.insert(*ERC777_MINTED_SIGNATURE);
        topics.insert(*ERC777_BURNED_SIGNATURE);
        topics.insert(*ERC777_AUTHORIZED_OPERATOR_SIGNATURE);
        topics.insert(*ERC777_REVOKED_OPERATOR_SIGNATURE);
    }

    // Convert HashSet back to Vec<H256> for the FilterBuilder
    let topics_vec: Vec<H256> = topics.into_iter().collect();
    info!("Fetching logs with topics: {:?}", topics_vec);
    // Build the log filter
    let filter = Filter::new()
        .from_block(BlockNumber::Number(from_block.into()))
        .to_block(BlockNumber::Number(to_block.into()))
        .topic0(topics_vec);

    let logs = match provider.get_logs(&filter).await {
        Ok(logs) => {
            info!("Fetched {} logs", logs.len());
            logs
        },
        Err(e) => {
            error!("Error fetching logs: {:?}", e);
            Vec::new()
        },
    };
    logs
}
