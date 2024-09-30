use diesel::prelude::*;
use crate::schema::allowances::dsl::*;
use crate::PgPooledConnection;
use ethers::types::U256;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::allowances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Allowance {
    pub owner_address: Vec<u8>,     // 20-byte address
    pub spender_address: Vec<u8>,   // 20-byte address
    pub token_address: Vec<u8>,     // 20-byte address
    pub allowance: Option<String>,     // Allowance for ERC20, None for ERC721
    pub block_number: i32,          // Block number of the update
    pub token_id: Option<i16>,      // Token ID for ERC721/ERC1155, None for ERC20/ERC777
    pub token_type: String,         // Token type ("ERC20", "ERC721", "ERC1155", "ERC777")
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::allowances)]
pub struct NewAllowance<'a> {
    pub owner_address: &'a [u8],
    pub spender_address: &'a [u8],
    pub token_address: &'a [u8],
    pub allowance: Option<String>,
    pub block_number: i32,
    pub token_id: Option<i16>,
    pub token_type: &'a str,
}

pub fn update_historical_allowance(
    conn: &mut PgPooledConnection,
    owner: &[u8],               // 20-byte owner address
    spender: &[u8],             // 20-byte spender address
    token: &[u8],               // 20-byte token address
    value: String,              // Amount to add or subtract as a string
    token_id_value: Option<i16>,// Token ID for ERC721/1155, None for ERC20/ERC777
    token_type_value: &str,     // Token type (ERC20, ERC721, etc.)
    block_number_value: i32     // Block number as u32
) -> QueryResult<usize> {
    // Parse the provided value into U256
    let value_u256 = U256::from_dec_str(&value).unwrap_or_else(|_| U256::zero());

    // Get the most recent allowance for the owner, spender, and token
    let latest_allowance = allowances
        .filter(owner_address.eq(owner))
        .filter(spender_address.eq(spender))
        .filter(token_address.eq(token))
        .filter(token_id.eq(token_id_value))
        .order_by(block_number.desc())
        .first::<Allowance>(conn)
        .optional()?; // Get the latest allowance or return None if no record exists

    // Determine the new allowance value
    let new_allowance_value = if let Some(allowance_record) = latest_allowance {
        let current_allowance = U256::from_dec_str(&allowance_record.allowance.unwrap_or_else(|| "0".to_string()))
            .unwrap_or_else(|_| U256::zero());
        
        // Add or subtract the new value from the previous allowance
        if value.starts_with('-') {
            current_allowance.saturating_sub(value_u256) // Subtract, using saturating subtraction to prevent overflow
        } else {
            current_allowance.saturating_add(value_u256) // Add the new value
        }
    } else {
        value_u256 // If no previous allowance, the new allowance is the initial value
    };

    // Convert the new allowance value to a string
    let new_allowance_str = new_allowance_value.to_string();

    // Insert the new allowance record using a struct
    let new_allowance = NewAllowance {
        owner_address: owner,
        spender_address: spender,
        token_address: token,
        allowance: Some(new_allowance_str),
        block_number: block_number_value,
        token_id: token_id_value,
        token_type: token_type_value,
    };

    diesel::insert_into(allowances)
        .values(&new_allowance)
        .execute(conn)
}