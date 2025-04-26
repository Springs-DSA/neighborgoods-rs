mod models;
mod schema;
mod services;
mod utils;
mod routes;
mod db;


#[macro_use]
extern crate rocket;

use db::Db;
use chrono::Utc;
use rocket::request::FromParam;
use dotenvy::dotenv;
use models::{init, item};
use models::user::User;
use models::item::Item;
use models::item_transfer::{ItemTransfer, TransferPurpose, TransferStatus};
// use rocket::request::{self, Request, Outcome, FromRequest};
// use schema::users;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::{fairing::{self, AdHoc}, Rocket, Build};
use routes::item_transfers::{item_transfer_post, item_transfer_put, item_transfers_get};
use routes::tags::{tag_create, item_tag_assign, item_tag_revoke};
use schema::{item_transfers, items, node_settings, users};
use uuid::Uuid;
use std::env;
use std::ops::DerefMut;
use std::str::FromStr;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
// use utils::password;
// use rocket::http::{Cookie, CookieJar, Status};
use routes::signup::{signup_get, signup_post};
use routes::login::{login_get, login_post, logout};
use routes::dashboard::{dashboard_get, dashboard_redirect};
use routes::items::{inventory_get, item_delete, item_get, items_contribute_get, items_contribute_post};

use rocket::fs::{FileServer, relative, TempFile};
use std::fs;
use services::item_service::{ItemViewContext, get_item_view_context};

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
            inventory_get, items_contribute_get, items_contribute_post, item_get, item_delete,
            item_transfer_post, item_transfers_get, item_transfer_put,
            tag_create, item_tag_assign, item_tag_revoke
        ])
        .mount("/public", FileServer::from(relative!("uploads")))
        .attach(Template::fairing())
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Initialize Node Settings", run_migrations))
}

// next TODO:
// 1. clean up and move previous work to appropriate files - DONE
// 2. page for contributing a new item. - DONE
// 3. list of your contributed items
// 4. list of all available items
// 5. Borrow an item
// 6. list of all your currently borrowed items
// 7. create issues for: only log in if approved, handle updating users if signup form is re-submitted before approval, validation of password, email, phone requirements, actual about page/info (settable from a node setting), map picker for lat and lng, include with signup, prevent going to login/signup pages when already logged in, move styling to the same file and consolidate, redirects if not authorized on other pages, async across the board on routes.
// 8. COMMUNITY AGREEMENT
// 9. dashboard link on navbar, separate navbar component
// 10. items have multiple images/files uploaded to them.
// 11. items should have tags instead of categories
// 12. should actually do something with the confirmation checkbox
// 13. landing page should redirect to the dashboard when logged in. 
// 14. files should be compressed when uploaded (thumbnail versions of pictures.)
