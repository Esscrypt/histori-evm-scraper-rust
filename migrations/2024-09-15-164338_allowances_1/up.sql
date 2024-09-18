-- up.sql
-- Create the enum for token types if not already created
-- Modify the tokens table
CREATE TABLE tokens (
    token_address BYTEA PRIMARY KEY,           -- 20-byte token address (unique for each token)
    block_number INTEGER NOT NULL,             -- Integer for block number
    token_type VARCHAR NOT NULL,       -- Enum to represent the type of token

    -- ERC20-specific metadata (name and symbol required)
    name VARCHAR(255) NOT NULL,                -- Token name (required)
    symbol VARCHAR(50) NOT NULL,               -- Token symbol (required)
    decimals SMALLINT,                         -- Token decimals (optional for ERC20)
    granularity BIGINT 
);

-- Create the token_ids table to store ERC721/1155 token-specific metadata
CREATE TABLE token_ids (
    contract_address BYTEA NOT NULL REFERENCES tokens(token_address),  -- Contract address as a foreign key from tokens table
    token_id SMALLINT NOT NULL,                                          -- Token ID for ERC721/1155 tokens
    token_uri VARCHAR(255),                                            -- Token URI (Optional)
    PRIMARY KEY (contract_address, token_id)                           -- Unique per contract and token ID
);

-- Indexes for faster lookups
CREATE INDEX idx_token_ids_contract_address ON token_ids (contract_address);
CREATE INDEX idx_token_ids_token_id ON token_ids (token_id);

CREATE TABLE allowances (
    owner_address BYTEA NOT NULL,      -- 20 bytes, indexed
    spender_address BYTEA NOT NULL,    -- 20 bytes, indexed
    token_address BYTEA NOT NULL REFERENCES tokens(token_address), -- 20 bytes, indexed
    allowance BIGINT,                  -- Optional, NULL for ERC721
    block_number INTEGER NOT NULL,      -- Indexed for querying by block number
    token_id SMALLINT,                   -- Optional, for ERC721 tokens
    token_type VARCHAR NOT NULL,       -- Store "ERC20", "ERC721", etc.
    PRIMARY KEY (owner_address, token_address, block_number)  -- Unique per wallet, token, and block
);

-- Indexes for faster querying
CREATE INDEX idx_owner_address ON allowances (owner_address);
CREATE INDEX idx_spender_address ON allowances (spender_address);
CREATE INDEX idx_token_address ON allowances (token_address);
CREATE INDEX idx_block_number ON allowances (block_number);

CREATE TABLE token_supplies (
    token_address BYTEA NOT NULL REFERENCES tokens(token_address),  -- Token address as foreign key
    total_supply BIGINT NOT NULL,                                   -- Total supply of the token
    block_number INTEGER NOT NULL,                                  -- Block number at the time of snapshot
    PRIMARY KEY (token_address, block_number)                       -- Unique pair of token_address and block_number
);

-- Indexes for faster lookups
CREATE INDEX idx_token_supply_address ON token_supplies (token_address);
CREATE INDEX idx_token_supply_block ON token_supplies (block_number);

CREATE TABLE balances (
    wallet_address BYTEA NOT NULL,                                  -- 20-byte wallet address
    token_address BYTEA NOT NULL REFERENCES tokens(token_address),  -- 20-byte token address
    balance BIGINT NOT NULL,                                        -- Token balance for the wallet
    token_id SMALLINT,                                              -- Optional, for ERC721 tokens
    token_type VARCHAR NOT NULL,                                    -- Store "ERC20", "ERC721", etc.
    block_number INTEGER NOT NULL,                                  -- Block number at the time of balance update
    PRIMARY KEY (wallet_address, token_address, block_number)       -- Unique per wallet, token, and block
);

-- Indexes for faster querying
CREATE INDEX idx_balance_wallet ON balances (wallet_address);
CREATE INDEX idx_balance_token ON balances (token_address);
CREATE INDEX idx_balance_block ON balances (block_number);
