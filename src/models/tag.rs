use super::super::schema::tags;
use rocket_db_pools::diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
