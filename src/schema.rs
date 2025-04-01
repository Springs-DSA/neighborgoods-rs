// @generated automatically by Diesel CLI.

diesel::table! {
    node_settings (id) {
        id -> Int4,
        entity -> Varchar,
        attribute -> Varchar,
        value -> Text,
        data_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        lat -> Nullable<Numeric>,
        lng -> Nullable<Numeric>,
        home_node_id -> Nullable<Varchar>,
        password_hash -> Varchar,
        password_reset_token -> Nullable<Varchar>,
        password_reset_expiration -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    node_settings,
    users,
);
