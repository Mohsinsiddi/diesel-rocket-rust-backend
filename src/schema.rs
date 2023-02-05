// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Varchar,
        collection_name -> Varchar,
        ceiling_price -> Int4,
        active_trades -> Int4,
        total_trades -> Int4,
        volume -> Int4,
        supply -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Varchar,
        trade_id -> Varchar,
        collection_id -> Varchar,
        trade_amount -> Int4,
        rarity -> Varchar,
        collection_root -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Varchar,
        role_name -> Varchar,
    }
}

diesel::table! {
    trades (id) {
        id -> Varchar,
        title -> Varchar,
        content -> Text,
        created_by -> Varchar,
        accepted_order_id -> Int4,
        deposited_amount -> Int4,
        buyer_address -> Varchar,
        seller_address -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        email -> Varchar,
        role_id -> Varchar,
        password -> Varchar,
        address -> Varchar,
        user_name -> Varchar,
    }
}

diesel::joinable!(orders -> collections (collection_id));
diesel::joinable!(orders -> trades (trade_id));
diesel::joinable!(trades -> users (created_by));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    collections,
    orders,
    roles,
    trades,
    users,
);
