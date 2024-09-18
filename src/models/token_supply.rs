use diesel::prelude::*;
use crate::schema::token_supplies::dsl::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::token_supply)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenSupply {
    pub token_address: Vec<u8>,   // Token address (20 bytes)
    pub total_supply: i64,        // Total supply of the token
    pub block_number: i32,        // Block number when the supply was recorded
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::token_supplies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTokenSupply<'a> {
    pub token_address: &'a [u8],  // 20-byte token address
    pub total_supply: i64,        // Token's total supply
    pub block_number: i32,        // Block number of the snapshot
}

pub fn update_total_supply(
    conn: &mut PgConnection,
    token_address_value: &[u8],   // 20-byte token address
    value: i64,             // Amount to add or subtract from total supply
    block_number_value: i32       // Block number
) -> QueryResult<usize> {
    // Get the most recent total supply for the token
    let latest_total_supply = token_supplies
        .filter(token_address.eq(token_address_value))
        .order_by(block_number.desc())  // Get the latest total supply by block number
        .first::<TokenSupply>(conn)
        .optional()?;  // Get the latest total supply or return None if no record exists

    let new_total_supply_value = if let Some(supply_record) = latest_total_supply {
        supply_record.total_supply + value  // Add or subtract from the previous total supply
    } else {
        value  // If no previous total supply, the new total supply is the initial value
    };

    // Insert a new total supply record
    let new_total_supply = NewTokenSupply {
        token_address: token_address_value,
        total_supply: new_total_supply_value,
        block_number: block_number_value,
    };

    diesel::insert_into(token_supplies)
        .values(&new_total_supply)
        .execute(conn)
}

