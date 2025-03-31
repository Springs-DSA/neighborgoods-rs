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
async fn list(mut db: Connection<DbConn>) -> Template {
    let results = node_settings::table
        .load::<models::node_settings::NodeSettings>(&mut db)
        .await;

    match results {
        Ok(settings) => {
            let context = context! { settings };
            Template::render("landing", &context)
        }
        Err(_) => Template::render("error", context! { error: "Failed to load settings" }),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    
    rocket::build()
        .mount("/", routes![hello, list])
        .attach(Template::fairing())
        .attach(DbConn::init())
}
