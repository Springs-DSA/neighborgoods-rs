use super::super::schema::item_tags;
use rocket_db_pools::diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = item_tags)]
pub struct ItemTag {
    pub item_id: Uuid,
    pub tag: String,
    pub created_at: chrono::NaiveDateTime,
}
