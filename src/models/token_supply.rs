use diesel::prelude::*;
use crate::schema::token_supplies::dsl::*;

use ethers::types::U256;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::token_supply)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenSupply {
    pub token_address: Vec<u8>,   // Token address (20 bytes)
    pub total_supply: String,        // Total supply of the token
    pub block_number: i32,        // Block number when the supply was recorded
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::token_supplies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTokenSupply<'a> {
    pub token_address: &'a [u8],  // 20-byte token address
    pub total_supply: String,        // Token's total supply
    pub block_number: i32,        // Block number of the snapshot
}

pub fn update_total_supply(
    conn: &mut PgConnection,
    token_address_value: &[u8],   // 20-byte token address
    value: String,                // Amount to add or subtract as a string
    block_number_value: i32       // Block number
) -> QueryResult<usize> {
    // Get the most recent total supply for the token
    let latest_total_supply = token_supplies
        .filter(token_address.eq(token_address_value))
        .order_by(block_number.desc())  // Get the latest total supply by block number
        .first::<TokenSupply>(conn)
        .optional()?;  // Get the latest total supply or return None if no record exists

    // Convert the `value` to `U256`
    let value_u256 = U256::from_dec_str(&value).unwrap_or_else(|_| U256::zero());

    // Determine the new total supply value
    let new_total_supply_value = if let Some(supply_record) = latest_total_supply {
        let current_supply = U256::from_dec_str(&supply_record.total_supply).unwrap_or_else(|_| U256::zero());

        // Add or subtract the new value from the previous total supply
        if value.starts_with('-') {
            current_supply.saturating_sub(value_u256) // Subtract, using saturating subtraction to prevent overflow
        } else {
            current_supply.saturating_add(value_u256) // Add the new value
        }
    } else {
        value_u256 // If no previous total supply, the new total supply is the initial value
    };

    // Convert the new total supply to a string
    let new_total_supply_str = new_total_supply_value.to_string();

    // Insert a new total supply record
    let new_total_supply = NewTokenSupply {
        token_address: token_address_value,
        total_supply: new_total_supply_str, // The updated total supply as a string
        block_number: block_number_value,
    };

    diesel::insert_into(token_supplies)
        .values(&new_total_supply)
        .execute(conn)
}