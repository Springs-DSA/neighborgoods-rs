// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transfer_purpose"))]
    pub struct TransferPurpose;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transfer_status"))]
    pub struct TransferStatus;
}

diesel::table! {
    certifications (id) {
        id -> Uuid,
        code -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        lifetime -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransferPurpose;

    item_cert_requirements (item_id, cert_id) {
        item_id -> Uuid,
        cert_id -> Uuid,
        purposes -> Array<Nullable<TransferPurpose>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransferPurpose;
    use super::sql_types::TransferStatus;

    item_transfers (id) {
        id -> Uuid,
        item_id -> Uuid,
        steward_id -> Uuid,
        prev_steward_id -> Nullable<Uuid>,
        purpose -> TransferPurpose,
        lat -> Nullable<Numeric>,
        lng -> Nullable<Numeric>,
        status -> TransferStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    items (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        contributed_by -> Uuid,
        upload_directory_path -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

diesel::table! {
    user_certifications (user_id, cert_id) {
        user_id -> Uuid,
        cert_id -> Uuid,
        expires_at -> Timestamp,
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

diesel::joinable!(item_cert_requirements -> certifications (cert_id));
diesel::joinable!(item_cert_requirements -> items (item_id));
diesel::joinable!(item_transfers -> items (item_id));
diesel::joinable!(items -> users (contributed_by));
diesel::joinable!(user_certifications -> certifications (cert_id));
diesel::joinable!(user_certifications -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    certifications,
    item_cert_requirements,
    item_transfers,
    items,
    node_settings,
    user_certifications,
    users,
);
