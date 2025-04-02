use super::super::schema::certifications;
use rocket_db_pools::diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = certifications)]
pub struct Certification {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub lifetime: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
