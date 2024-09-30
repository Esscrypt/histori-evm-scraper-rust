// @generated automatically by Diesel CLI.

diesel::table! {
    allowances (id) {
        id -> Int4,
        owner_address -> Bytea,
        spender_address -> Bytea,
        token_address -> Bytea,
        allowance -> Nullable<Text>,
        block_number -> Int4,
        token_id -> Nullable<Int2>,
        token_type -> Text,
    }
}

diesel::table! {
    balances (id) {
        id -> Int4,
        wallet_address -> Bytea,
        token_address -> Bytea,
        balance -> Text,
        token_id -> Nullable<Int2>,
        token_type -> Text,
        block_number -> Int4,
    }
}

diesel::table! {
    token_ids (id) {
        id -> Int4,
        contract_address -> Bytea,
        token_id -> Int2,
        token_uri -> Nullable<Text>,
    }
}

diesel::table! {
    token_supplies (id) {
        id -> Int4,
        token_address -> Bytea,
        total_supply -> Text,
        block_number -> Int4,
    }
}

diesel::table! {
    tokens (token_address) {
        token_address -> Bytea,
        block_number -> Int4,
        token_type -> Varchar,
        name -> Nullable<Text>,
        symbol -> Nullable<Text>,
        decimals -> Nullable<Int2>,
        granularity -> Nullable<Text>,
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
