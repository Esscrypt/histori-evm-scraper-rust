use crate::schema::balances::dsl::*;
use crate::PgPooledConnection;
use diesel::prelude::*;
use ethers::types::U256;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Balance {
    pub id: i32,                // Unique ID for the balance record
    pub wallet_address: Vec<u8>, // Wallet address (20 bytes)
    pub token_address: Vec<u8>,  // Token address (20 bytes)
    pub balance: String,            // Token balance (amount for ERC20/1155, 1 for ERC721)
    pub token_id: Option<i16>,   // Token ID for ERC721/1155, NULL for ERC20
    pub block_number: i32,       // Block number when balance was last updated
    pub token_type: String,      // "ERC20", "ERC721", "ERC1155", etc.
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::balances)]
pub struct NewBalance<'a> {
    pub wallet_address: &'a [u8], // Wallet address (20 bytes)
    pub token_address: &'a [u8],  // Token address (20 bytes)
    pub balance: String,             // Token balance (amount for ERC20/1155, 1 for ERC721)
    pub token_id: Option<i16>,    // Token ID for ERC721/1155, NULL for ERC20
    pub block_number: i32,        // Block number when balance was last updated
    pub token_type: &'a str,      // "ERC20", "ERC721", "ERC1155", etc.
}

pub fn update_historical_balance(
    conn: &mut PgPooledConnection, // Use PooledConnection instead of PgConnection
    wallet: &[u8],                 // 20-byte wallet address
    token: &[u8],                  // 20-byte token address
    value: String,                 // Amount to add or subtract as a string
    token_id_value: Option<i16>,   // Token ID for ERC721/1155, None for ERC20
    token_type_value: &str,        // Token type (ERC20, ERC721, etc.)
    block_number_value: i32,       // Block number
) -> QueryResult<usize> {
    let mut query = balances
        .filter(wallet_address.eq(wallet))
        .filter(token_address.eq(token))
        .order_by(block_number.desc())
        .into_boxed(); // Use `.into_boxed()` to allow conditional filters

    if let Some(other_id) = token_id_value {
        query = query.filter(token_id.eq(other_id)); // Add the token_id filter if it's Some
    }

    // Get the latest balance
    let latest_balance: Option<Balance> = query.select(Balance::as_select()).first::<Balance>(conn).optional()?;

    // Convert the `value` to `U256`
    let value_u256 = U256::from_dec_str(&value).unwrap_or_else(|_| U256::zero());

    // Determine the new balance value
    let new_balance_value = if let Some(balance_record) = latest_balance {
        let current_balance = U256::from_dec_str(&balance_record.balance).unwrap_or_else(|_| U256::zero());

        // Add or subtract the new value from the previous balance
        if value.starts_with('-') {
            current_balance.saturating_sub(value_u256) // Subtract, using saturating subtraction to prevent overflow
        } else {
            current_balance.saturating_add(value_u256) // Add the new value
        }
    } else {
        value_u256 // If no previous balance, the new balance is the initial value
    };

    // Convert the new balance to a string
    let new_balance_str = new_balance_value.to_string();

    // Insert the new balance record
    let new_balance = NewBalance {
        wallet_address: wallet,
        token_address: token,
        balance: new_balance_str, // The updated balance as a string
        token_id: token_id_value,
        block_number: block_number_value,
        token_type: token_type_value,
    };

    diesel::insert_into(balances)
        .values(&new_balance)
        .execute(conn)
}