// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Binary,
        amount -> Text,
        recipient -> Binary,
        sender -> Binary,
        timestamp -> TimestamptzSqlite,
    }
}

diesel::table! {
    users (id) {
        id -> Binary,
        username -> Text,
        password_hash -> Text,
        balance -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(transactions, users,);
