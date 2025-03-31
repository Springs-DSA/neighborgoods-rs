use super::super::schema::node_settings;
use rocket_db_pools::diesel:: prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = node_settings)]
pub struct NodeSettings {
    pub id: i32,
    pub entity: String,
    pub attribute: String,
    pub value: String,
    pub data_type: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}