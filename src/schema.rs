// @generated automatically by Diesel CLI.

diesel::table! {
    locations (w3w) {
        w3w -> Varchar,
        lat -> Numeric,
        lng -> Numeric,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    locations,
    node_settings,
);
