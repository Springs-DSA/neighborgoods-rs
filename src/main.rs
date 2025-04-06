mod models;
mod schema;
mod services;
mod utils;
mod routes;


#[macro_use]
extern crate rocket;

use chrono::{NaiveDateTime, Utc};
use dotenvy::dotenv;
use models::init;
use models::user::User;
use schema::users;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{PgPool, prelude::*};
use rocket_dyn_templates::{Template, context};
use rocket::{fairing::{self, AdHoc}, Rocket, Build};
use schema::node_settings;
use uuid::uuid;
use std::env;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use utils::password;
// use routes::signup::signup_page;

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
        .mount("/", routes![hello, landing, signup_get, signup_post])
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


#[derive(FromForm)]
struct UserData<'r> {
    r#username: &'r str,
    r#email: &'r str,
    r#password: &'r str,
    r#password_confirm: &'r str
}

#[get("/signup")]
fn signup_get() -> Template {
    let context = context! {};
    Template::render("signup", &context)
}

#[post("/signup", data = "<user>")]
async fn signup_post(user: Form<UserData<'_>>, mut db: Connection<Db>) -> Flash<Redirect> {

    // hash passwords
    if user.password != user.password_confirm {
        return Flash::error(Redirect::to("/signup"), "Passwords do not match!")
    } 

    let hashed_password = password::hash_password(user.password);
    match hashed_password {
        Ok(hp) => {

            let now = Utc::now().naive_utc();

            // Insert the new user into the database
            let new_user = User {
                id: uuid::Uuid::new_v4(),
                name: user.username.into(),
                email: user.email.into(),
                password_hash: hp.clone(),
                phone: None,
                lat: None,
                lng: None,
                home_node_id: Some(env::var("NODE_ID").expect("NODE_ID must be set in the environment variables")),
                password_reset_token: None,
                password_reset_expiration: None,
                created_at: now.clone(),
                updated_at: now.clone(),
                approved_at: None,
                approved_by: None

            };
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut db)
                .await
                .expect("Failed to create user");
        }
        Err(_) => {
            return Flash::error(Redirect::to("/signup"), "Password Hash failed");
        }
    }


    Flash::success(Redirect::to("/"), "User created! Approval pending...")
}