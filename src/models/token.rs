use diesel::prelude::*;

/// Struct to represent a token in the database, including metadata for ERC20 and ERC721.
#[derive(Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub token_address: Vec<u8>,            // 20-byte token address
    pub block_number: i32,                 // Integer for block number
    pub token_type: String,             // Enum for the token type (ERC20, ERC721, etc.)
    pub name: Option<String>,                      // Name is required
    pub symbol: Option<String>,                    // Symbol is required
    
    // ERC20-specific metadata
    pub decimals: Option<i16>,             // Decimals (optional for ERC20)

    // ERC777-specific metadata
    pub granularity: Option<i64>,             // Granularity (optional for ERC777)

}

/// Struct to represent new token data to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewToken<'a> {
    pub token_address: &'a [u8],           // Token address as bytes
    pub block_number: i32,                 // Block number
    pub token_type: &'a str,             // Enum for the token type (ERC20, ERC721, etc.)
    
    pub name: Option<String>,                     // Name is required
    pub symbol: Option<String>,                   // Symbol is required
    pub decimals: Option<i16>,             // Decimals (optional for ERC20)
    pub granularity: Option<i64>,             // Decimals (optional for ERC20)
    
}