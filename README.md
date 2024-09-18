# EVM Scraper

This project scrapes and processes Ethereum logs for various token standards (ERC20, ERC721, ERC1155, ERC777) and writes historical balances, allowances, metadata, and total supplies to a PostgreSQL database.

### Prerequisites

- Rust installed: https://www.rust-lang.org/
- PostgreSQL installed and running
- Set up `.env` with `DATABASE_URL`

### Setup

1. Clone the repository:

```bash
git clone https://github.com/esscrypt/histori-evm-scraper-rust.git
cd evm_scraper
cargo build

```