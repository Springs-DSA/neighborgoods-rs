use super::super::schema::item_cert_requirements;
use crate::models::item_transfer::TransferPurpose;
use rocket_db_pools::diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = item_cert_requirements)]
pub struct ItemCertRequirement {
    pub item_id: Uuid,
    pub cert_id: Uuid,
    pub purposes: Vec<Option<TransferPurpose>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
