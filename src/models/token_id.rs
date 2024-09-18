use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::token_ids)]
pub struct TokenID {
    pub contract_address: Vec<u8>,
    pub token_id: i16,
    pub token_uri: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::token_ids)]
pub struct NewTokenID<'a> {
    pub contract_address: &'a [u8],  // Wallet address (20 bytes)
    pub token_id: i16,   // Token address (20 bytes)
    pub token_uri: Option<String>,  // Token uri (amount for ERC20/1155, 1 for ERC721)
}