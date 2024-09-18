-- down.sql

-- Drop the allowances table and associated indexes
DROP INDEX IF EXISTS idx_owner_address;
DROP INDEX IF EXISTS idx_spender_address;
DROP INDEX IF EXISTS idx_token_address;
DROP INDEX IF EXISTS idx_block_number;
DROP TABLE IF EXISTS allowances;

-- Drop the balances table and associated indexes
DROP INDEX IF EXISTS idx_balance_wallet;
DROP INDEX IF EXISTS idx_balance_token;
DROP INDEX IF EXISTS idx_balance_block;
DROP TABLE IF EXISTS balances;

-- Drop the token_supplies table and associated indexes
DROP INDEX IF EXISTS idx_token_supply_address;
DROP INDEX IF EXISTS idx_token_supply_block;
DROP TABLE IF EXISTS token_supplies;

-- Drop the token_ids table and associated indexes
DROP INDEX IF EXISTS idx_token_ids_contract_address;
DROP INDEX IF EXISTS idx_token_ids_token_id;
DROP TABLE IF EXISTS token_ids;

-- Drop the tokens table with CASCADE to remove dependencies
DROP TABLE IF EXISTS tokens CASCADE;

-- Drop the enum type for token types
DROP TYPE IF EXISTS token_type_enum;
