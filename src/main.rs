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
use dotenvy::dotenv;
use models::init;
use models::user::User;
use models::item::Item;
// use rocket::request::{self, Request, Outcome, FromRequest};
// use schema::users;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::{fairing::{self, AdHoc}, Rocket, Build};
use schema::{node_settings, items};
use uuid::Uuid;
use std::env;
use std::fmt::format;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
// use utils::password;
// use rocket::http::{Cookie, CookieJar, Status};
use routes::signup::{signup_get, signup_post};
use routes::login::{login_get, login_post, logout};
use routes::dashboard::{dashboard_get, dashboard_redirect};

use rocket::fs::TempFile;
use std::fs;

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
            inventory_get, items_contribute_get, items_contribute_post
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
// 7. create issues for: only log in if approved, handle updating users if signup form is re-submitted before approval, validation of password, email, phone requirements, actual about page/info (settable from a node setting), map picker for lat and lng, include with signup, prevent going to login/signup pages when already logged in, move styling to the same file and consolidate, redirects if not authorized on other pages, async across the board on routes.
// 8. COMMUNITY AGREEMENT
// 9. dashboard link on navbar, separate navbar component
// 10. items have multiple images/files uploaded to them.


#[get("/inventory")]
pub fn inventory_get(user: User) -> Template {
    let context = context! {
        user
    };
    Template::render("inventory", &context)
}

#[derive(FromForm, Debug)]
pub struct ItemData<'r> {
    r#name: &'r str,
    r#description: &'r str,
    r#category: &'r str,
    image: TempFile<'r>,
}

#[get("/items/contribute")]
pub fn items_contribute_get(user: User) -> Template {
    let context = context! {
        user
    };
    Template::render("item_contribution", &context)
}

#[post("/items/contribute", data = "<item>")]
pub async fn items_contribute_post(user: User, mut item: Form<ItemData<'_>>, mut db: Connection<Db>) -> Flash<Redirect> {
    println!("ItemData: {:?}", item);
    // create the item in the database
    let item_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();
    let new_item = Item {
        id: item_id,
        name: item.name.into(),
        description: Some(item.description.into()),
        contributed_by: user.id,
        upload_directory_path: format!("uploads/{}", item_id.to_string()),
        created_at: now.clone(),
        updated_at: now.clone()
    };

    // create item upload dir.
    let dir_path = format!("uploads/{}", item_id.to_string());
    fs::create_dir(dir_path).unwrap();

    // upload item
    if let Some(name) = item.image.name() {

        let path = format!("uploads/{}/{}", item_id.to_string(), name);
        
        let upload_result = item.image.persist_to(path).await;
        match upload_result {
            Ok(_) => {
                println!(">>> file path: {}", item.image.path().unwrap().to_str().unwrap());
            },
            Err(e) => {
                println!("ERROR: {}", e); // need to have full path specified.
                return Flash::error(Redirect::to("/inventory"), "Item file(s) couldn't be uploaded");
            }
        }
    }

    // if we've arrived here, then we have successfully uploaded.

    let db_result = diesel::insert_into(items::table)
        .values(&new_item)
        .execute(&mut db)
        .await;
    
    match db_result {
        Ok(_) => Flash::success(Redirect::to("/inventory"), "Item successfully created"),
        Err(e) => {
            println!("ERROR: {}", e);
            //TODO remove uploaded folder/files
            Flash::error(Redirect::to("/inventory"), "Item record couldn't be created")
        },
    }
}