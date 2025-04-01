use super::super::schema::locations;
use bigdecimal::BigDecimal;
use rocket_db_pools::diesel:: prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = locations)]
pub struct Locations {
    pub w3w: String,
    pub lat: BigDecimal,
    pub lng: BigDecimal,
}