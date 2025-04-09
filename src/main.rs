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
use models::item_transfer::{ItemTransfer, TransferPurpose, TransferStatus};
// use rocket::request::{self, Request, Outcome, FromRequest};
// use schema::users;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::{fairing::{self, AdHoc}, Rocket, Build};
use schema::{item_transfers, items, node_settings, users};
use uuid::Uuid;
use std::env;
use std::ops::DerefMut;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
// use utils::password;
// use rocket::http::{Cookie, CookieJar, Status};
use routes::signup::{signup_get, signup_post};
use routes::login::{login_get, login_post, logout};
use routes::dashboard::{dashboard_get, dashboard_redirect};

use rocket::fs::{FileServer, relative, TempFile};
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
            inventory_get, items_contribute_get, items_contribute_post, item_get
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


#[get("/inventory")]
pub async fn inventory_get(user: User, mut db: Connection<Db>) -> Template {
    // get list of items
    let items = items::table
        .inner_join(users::table)
        .load::<(Item, User)>(&mut db)
        .await
        .expect("Error loading items");
    let context = context! {
        items,
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
pub async fn items_contribute_get(user: User) -> Template {
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
    let mut new_item = Item {
        id: item_id,
        name: item.name.into(),
        description: Some(item.description.into()),
        contributed_by: user.id,
        upload_directory_path: format!("public/{}/", item_id.to_string()),
        created_at: now.clone(),
        updated_at: now.clone()
    };

    // create item upload dir.
    let dir_path = format!("uploads/{}", item_id.to_string());
    fs::create_dir(dir_path).unwrap();

    // upload item
    if let Some(name) = item.deref_mut().image.name() {

        let path = format!("uploads/{}/{}", item_id.to_string(), name);
        // attach the image file to the end of the upload_directory_path
        // TODO need to be able to handle all the files in the directory, not just one
        new_item.upload_directory_path.push_str(name);
        
        let upload_result = item.image.persist_to(path).await;
        match upload_result {
            Ok(_) => {
                println!(">>> file path: {}", item.image.path().unwrap().to_str().unwrap());
                println!(">>> item upload dir path: {}", new_item.upload_directory_path);
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
        Ok(_) => {
            // also, create the first item transfer to move the item to the contributing user's stewardship.

            let initial_item_transfer = ItemTransfer { 
                id: Uuid::new_v4(),
                item_id,
                steward_id: user.id,
                prev_steward_id: None,
                purpose: TransferPurpose::Contribute,
                lat: None,
                lng: None,
                status: TransferStatus::Completed,
                created_at: now.clone(),
                updated_at: now.clone()
            };

            let iit_result = diesel::insert_into(item_transfers::table)
                .values(&initial_item_transfer)
                .execute(&mut db)
                .await;
            match iit_result {
                Ok(_) => {
                    println!("Successfully created item transfer");
                    Flash::success(Redirect::to("/inventory"), "Item successfully created")
                },
                Err(e) => {
                    // TODO delete item record, remove uploads.
                    // https://stackoverflow.com/questions/75939019/transactions-in-rust-diesel
                    println!(">>> couldn't create item transfer, {}", e);
                    Flash::error(Redirect::to("/inventory"), "Item record couldn't be created")
                }
            
            }

            
        },
        Err(e) => {
            println!("ERROR: {}", e);
            //TODO remove uploaded folder/files
            Flash::error(Redirect::to("/inventory"), "Item record couldn't be created")
        },
    }
}

// get an individual item's page.
#[get("/items/<item_id>")]
pub async fn item_get(user: User, item_id: Uuid, mut db: Connection<Db>) -> Template {
    // 1. Load just the item first
    let item_result = items::table
        .find(item_id)
        .get_result::<Item>(&mut db)
        .await;
    
    let item = match item_result {
        Ok(item) => item,
        Err(_) => return Template::render("error", context! { error: "Item not found" }),
    };
    
    // 2. Load the contributor
    let contributor_result = users::table
        .find(item.contributed_by)
        .get_result::<User>(&mut db)
        .await;
    
    let contributor = match contributor_result {
        Ok(contributor) => contributor,
        Err(_) => return Template::render("error", context! { error: "Contributor not found" }),
    };
    
    // 3. Load the 5 most recent transfers with their stewards
    let transfers_with_stewards = item_transfers::table
        .filter(item_transfers::item_id.eq(item_id))
        .inner_join(users::table.on(item_transfers::steward_id.eq(users::id)))
        .order(item_transfers::updated_at.desc())
        .limit(5)
        .load::<(ItemTransfer, User)>(&mut db)
        .await
        .expect("error getting item transfers"); // Default to empty vector if error
    
    // 4. Extract the current steward ID from the first transfer (if available)
    let current_steward_id = item_transfers::table
        .filter(item_transfers::item_id.eq(item_id))
        .inner_join(users::table.on(item_transfers::steward_id.eq(users::id)))
        .order(item_transfers::updated_at.desc())
        .first::<(ItemTransfer, User)>(&mut db)
        .await
        .expect("error getting item transfers")
        .1.id;
    
    // 5. Check if the current user is the steward
    let is_current_steward = current_steward_id == user.id;
    //     .map(|steward_id| steward_id == user.id)
    //     .unwrap_or(false);
    
    let context = context! {
        user,
        item,
        contributor,
        transfers_with_stewards,
        current_steward_id,
        is_current_steward
    };
    
    Template::render("item_view", &context)
}