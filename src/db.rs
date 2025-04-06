use rocket_db_pools::Database;
use rocket_db_pools::diesel::PgPool;

#[derive(Database)]
#[database("neighborgoods_db")]
pub struct Db(PgPool);