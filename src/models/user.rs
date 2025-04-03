use super::super::schema::users;
use rocket_db_pools::diesel::prelude::*;
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub lat: Option<BigDecimal>,
    pub lng: Option<BigDecimal>,
    pub home_node_id: Option<String>,
    pub password_hash: String,
    pub password_reset_token: Option<String>,
    pub password_reset_expiration: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub approved_at: Option<chrono::NaiveDateTime>,
    pub approved_by: Option<Uuid>,
}
