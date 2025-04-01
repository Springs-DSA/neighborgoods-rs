mod models;
mod schema;


#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use models::init;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{PgPool, prelude::*};
use rocket_dyn_templates::{Template, context};
use rocket::{fairing::{self, AdHoc}, Rocket, Build};
use schema::node_settings;
use std::env;

#[derive(Database)]
#[database("neighborgoods_db")]
struct Db(PgPool);

// Legacy route
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/")]
async fn landing(mut db: Connection<Db>) -> Template {
    let results = node_settings::table
        .filter(node_settings::entity.eq(env::var("NODE_ID").expect("NODE_ID must be set")))
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
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Initialize Node Settings", run_migrations))
}


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = Db::fetch(&rocket) {
        // Get a connection from the pool
        match db.get().await {
            Ok(mut conn) => {
                match init::initialize_node_settings(&mut conn).await {
                    Ok(_) => {
                        info!("Successfully initialized node settings");
                        Ok(rocket)
                    },
                    Err(e) => {
                        error!("Failed to initialize node settings: {}", e);
                        Err(rocket)
                    }
                }
            },
            Err(e) => {
                error!("Failed to get database connection: {}", e);
                Err(rocket)
            }
        }
    } else {
        error!("Failed to fetch database pool");
        Err(rocket)
    }
}
