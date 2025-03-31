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
