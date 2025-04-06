mod models;
mod schema;
mod services;
mod utils;
mod routes;
mod db;


#[macro_use]
extern crate rocket;

use db::Db;
// use chrono::Utc;
use dotenvy::dotenv;
use models::init;
// use models::user::User;
// use rocket::request::{self, Request, Outcome, FromRequest};
// use schema::users;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::{fairing::{self, AdHoc}, Rocket, Build};
use schema::node_settings;
// use uuid::Uuid;
use std::env;
// use rocket::form::Form;
// use rocket::response::{Flash, Redirect};
// use utils::password;
// use rocket::http::{Cookie, CookieJar, Status};
use routes::signup::{signup_get, signup_post};
use routes::login::{login_get, login_post, logout};
use routes::dashboard::{dashboard_get, dashboard_redirect};


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

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    
    rocket::build()
        .mount("/", routes![
            landing,
            signup_get,
            signup_post,
            login_get,
            login_post,
            logout,
            dashboard_get,
            dashboard_redirect,
        ])
        .attach(Template::fairing())
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Initialize Node Settings", run_migrations))
}

// next TODO:
// 2. page for contributing a new item.
// 3. list of your contributed items
// 4. list of all available items
// 5. Borrow an item
// 6. list of all your currently borrowed items
// 7. create issues for: only log in if approved, handle updating users if signup form is re-submitted before approval, validation of password, email, phone requirements, actual about page/info (settable from a node setting), map picker for lat and lng, include with signup, prevent going to login/signup pages when already logged in, move styling to the same file and consolidate.