// @generated automatically by Diesel CLI.

diesel::table! {
    allowances (owner_address, token_address, block_number) {
        owner_address -> Bytea,
        spender_address -> Bytea,
        token_address -> Bytea,
        allowance -> Nullable<Int8>,
        block_number -> Int4,
        token_id -> Nullable<Int2>,
        token_type -> Varchar,
    }
}

diesel::table! {
    balances (wallet_address, token_address, block_number) {
        wallet_address -> Bytea,
        token_address -> Bytea,
        balance -> Int8,
        token_id -> Nullable<Int2>,
        token_type -> Varchar,
        block_number -> Int4,
    }
}

diesel::table! {
    token_ids (contract_address, token_id) {
        contract_address -> Bytea,
        token_id -> Int2,
        #[max_length = 255]
        token_uri -> Nullable<Varchar>,
    }
}

diesel::table! {
    token_supplies (token_address, block_number) {
        token_address -> Bytea,
        total_supply -> Int8,
        block_number -> Int4,
    }
}

diesel::table! {
    tokens (token_address) {
        token_address -> Bytea,
        block_number -> Int4,
        token_type -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        decimals -> Nullable<Int2>,
        granularity -> Nullable<Int8>,
    }
}

diesel::joinable!(allowances -> tokens (token_address));
diesel::joinable!(balances -> tokens (token_address));
diesel::joinable!(token_ids -> tokens (contract_address));
diesel::joinable!(token_supplies -> tokens (token_address));

diesel::allow_tables_to_appear_in_same_query!(
    allowances,
    balances,
    token_ids,
    token_supplies,
    tokens,
);
