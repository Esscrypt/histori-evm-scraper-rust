# EVM Scraper

This project scrapes and processes Ethereum logs for various token standards (ERC20, ERC721, ERC1155, ERC777) and writes historical balances, allowances, metadata, and total supplies to a PostgreSQL database.

## Prerequisites

- Rust installed: https://www.rust-lang.org/
- PostgreSQL installed and running
- Set up `.env` with `DATABASE_URL`

## Setup

1. Clone the repository:

```bash
git clone https://github.com/esscrypt/histori-evm-scraper-rust.git
cd evm_scraper
cargo build --release

```

## Environment Setup

Create a .env file in the root directory of the project with the following content:
```bash
RPC_URL=https://your_rpc_url_here
BLOCK_RANGE=10000
DATABASE_URL=postgres://your_db_user:your_db_password@localhost/your_db_name
```
Replace the placeholders with your actual values.
## Usage

To run the CLI with all available options, use the following command:
```bash
cargo run --release -- --erc20 --erc721 --erc1155 --erc777 --process-balances --process-allowances --process-total-supplies --process-token-uri
```
### Available Flags

-	--erc20: Include ERC20 events in the scraping process.
- 	--erc721: Include ERC721 events in the scraping process.
-	--erc1155: Include ERC1155 events in the scraping process.
-	--erc777: Include ERC777 events in the scraping process.
- 	--process-balances: Process token balances.
- 	--process-allowances: Process token allowances.
- 	--process-total-supplies: Process token total supplies.
- 	--process-token-uri: Process token URIs (e.g., metadata).

You can customize the command by including only the flags you need.

### Examples

**Run the Scraper for ERC20 and ERC721 Only:**
```bash
cargo run --release -- --erc20 --erc721
```
**Run the Scraper for All Token Types and Process Balances**
```bash
cargo run --release -- --erc20 --erc721 --erc1155 --erc777 --process-balances
```
## Contributing

	1.	Fork the repository.
	2.	Create a new branch: git checkout -b feature/your-feature-name.
	3.	Make your changes.
	4.	Commit your changes: git commit -m 'Add some feature'.
	5.	Push to the branch: git push origin feature/your-feature-name.
	6.	Open a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.