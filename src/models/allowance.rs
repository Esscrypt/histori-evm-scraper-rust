use diesel::prelude::*;
use crate::schema::allowances::dsl::*;
use crate::PgPooledConnection;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::allowances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Allowance {
    pub owner_address: Vec<u8>,     // 20-byte address
    pub spender_address: Vec<u8>,   // 20-byte address
    pub token_address: Vec<u8>,     // 20-byte address
    pub allowance: Option<i64>,     // Allowance for ERC20, None for ERC721
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
    pub allowance: Option<i64>,
    pub block_number: i32,
    pub token_id: Option<i16>,
    pub token_type: &'a str,
}

pub fn update_historical_allowance(
    conn: &mut PgPooledConnection,
    owner: &[u8],       // 20-byte owner address
    spender: &[u8],     // 20-byte spender address
    token: &[u8],       // 20-byte token address
    value: i64,         // Amount to add or subtract from the allowance
    token_id_value: Option<i16>,  // Token ID for ERC721/1155, None for ERC20/ERC777
    token_type_value: &str,       // Token type (ERC20, ERC721, etc.)
    block_number_value: i32       // Block number as u32
) -> QueryResult<usize> {
    // Get the most recent allowance for the owner, spender, and token
    let latest_allowance = allowances
        .filter(owner_address.eq(owner))
        .filter(spender_address.eq(spender))
        .filter(token_address.eq(token))
        .filter(token_id.eq(token_id_value))
        .order_by(block_number.desc())
        .first::<Allowance>(conn)
        .optional()?; // Get the latest allowance or return None if no record exists

    let new_allowance_value = if let Some(allowance_record) = latest_allowance {
        allowance_record.allowance.unwrap_or(0) + value  // Add or subtract from the previous allowance
    } else {
        value  // If no previous allowance, the new allowance is the initial value
    };

    // Insert the new allowance record using a struct
    let new_allowance = NewAllowance {
        owner_address: owner,
        spender_address: spender,
        token_address: token,
        allowance: Some(new_allowance_value),
        block_number: block_number_value,
        token_id: token_id_value,
        token_type: token_type_value,
    };

    diesel::insert_into(allowances)
        .values(&new_allowance)
        .execute(conn)
}
