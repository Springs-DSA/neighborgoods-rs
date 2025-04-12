use super::super::schema::item_transfers;
use rocket_db_pools::diesel::prelude::*;
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum, FromFormField)]
#[db_enum(existing_type_path = "crate::schema::sql_types::TransferPurpose")]
pub enum TransferPurpose {
    Use,
    Maintain,
    Restock,
    Consume,
    Contribute,
    Delist,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[db_enum(existing_type_path = "crate::schema::sql_types::TransferStatus")]
pub enum TransferStatus {
    Reserved,
    Completed,
    Canceled,
}

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = item_transfers)]
pub struct ItemTransfer {
    pub id: Uuid,
    pub item_id: Uuid,
    pub steward_id: Uuid,
    pub prev_steward_id: Option<Uuid>,
    pub purpose: TransferPurpose,
    pub lat: Option<BigDecimal>,
    pub lng: Option<BigDecimal>,
    pub status: TransferStatus,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub steward_confirmed_at: Option<chrono::NaiveDateTime>,
    pub prev_steward_confirmed_at: Option<chrono::NaiveDateTime>,
}

// Diesel generates these joinable! instances only for cases where there is only a single relation between two tables based on a non-composite foreign key. All other cases require to specify an explicit the ON clause while constructing such joins.