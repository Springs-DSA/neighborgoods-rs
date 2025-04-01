use super::super::schema::items;
use rocket_db_pools::diesel::prelude::*;
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub contributed_by: Uuid,
    pub upload_directory_path: String,
    pub lat: Option<BigDecimal>,
    pub lng: Option<BigDecimal>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
