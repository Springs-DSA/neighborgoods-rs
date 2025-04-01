mod models;
mod schema;


#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{AsyncPgConnection, QueryResult, PgPool, prelude::*};
use rocket_dyn_templates::{Template, context};
use schema::node_settings;
// use std::env;

#[derive(Database)]
#[database("neighborgoods_db")]
struct DbConn(PgPool);

// Legacy route
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/")]
async fn landing(mut db: Connection<DbConn>) -> Template {
    let results = node_settings::table
        .filter(node_settings::entity.eq("node"))
        .filter(node_settings::attribute.eq_any(vec!["name", "description"]))
        .load::<models::node_settings::NodeSettings>(&mut db)
        .await;

    match results {
        Ok(settings) => {
            // Extract name and description from settings
            let node_name = settings.iter()
                .find(|s| s.attribute == "name")
                .map(|s| s.value.clone())
                .unwrap_or_else(|| "NeighborGoods Node".to_string());
                
            let node_description = settings.iter()
                .find(|s| s.attribute == "description")
                .map(|s| s.value.clone())
                .unwrap_or_else(|| "A local instance of NeighborGoods".to_string());
                
            // Create context with specific values
            let context = context! { 
                node_name,
                node_description
            };
            
            Template::render("landing", &context)
        }
        Err(_) => Template::render("error", context! { error: "Failed to load settings" }),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    
    rocket::build()
        .mount("/", routes![hello, landing])
        .attach(Template::fairing())
        .attach(DbConn::init())
}
