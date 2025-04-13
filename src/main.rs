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
            item_transfer_post, item_transfers_get, item_transfer_put
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

// pub async fn item_edit_get()
// pub async fn item_edit_put()

// for requesting an item, use a post or other informal means. For making a transfer, or putting a hold on an item, use item transfer get
// pub async fn item_request_get() -> Template {
//     todo!()
// }

// pub async fn item_request_post() -> Flash<Redirect> {
//     todo!()
// }


#[derive(FromForm, Debug)]
pub struct TransferRequest {
    purpose: TransferPurpose,
}

#[post("/items/<item_id>/transfer", data = "<transfer_request>")]
pub async fn item_transfer_post(user: User, item_id: Uuid, transfer_request: Form<TransferRequest>, mut db: Connection<Db>) -> Flash<Redirect> {
    // when a transfer is first made, its status will be reserved, and the most recent completed item transfer/user will need to be 
    // found to get the current steward, so they can be marked as the previous steward in the new item transfer.

    // get the item transfer and user
    let current_transfer = item_transfers::table
        .filter(item_transfers::item_id.eq(item_id))
        .filter(item_transfers::status.eq(TransferStatus::Completed))
        .inner_join(users::table.on(item_transfers::steward_id.eq(users::id)))
        .order(item_transfers::updated_at.desc())
        .first::<(ItemTransfer, User)>(&mut db)
        .await
        .expect("error getting item transfers");

    let now = Utc::now().naive_utc();
    // create the new item transfer
    let new_transfer = ItemTransfer {
        id: Uuid::new_v4(),
        item_id,
        steward_id: user.id,
        prev_steward_id: Some(current_transfer.1.id),
        purpose: transfer_request.into_inner().purpose,
        lat: None,
        lng: None,
        status: TransferStatus::Reserved,
        created_at: now.clone(),
        updated_at: now.clone(),
        steward_confirmed_at: None,
        prev_steward_confirmed_at: None
    };

    let transfer_result = diesel::insert_into(item_transfers::table)
        .values(&new_transfer)
        .execute(&mut db)
        .await;

    let item_url = format!("/items/{}", item_id);

    match transfer_result {
        Ok(_) => Flash::success(Redirect::to(item_url), "Item transfer requested!"),
        Err(_) => Flash::error(Redirect::to(item_url), "Item transfer was unable to be created"),
    }
}

#[get("/items/transfers")]
pub async fn item_transfers_get(user: User, mut db: Connection<Db>) -> Template {

    let my_reserved_item_transfers = item_transfers::table
        .filter(item_transfers::status.eq(TransferStatus::Reserved))
        .filter(item_transfers::steward_id.eq(user.id))  // User is the new steward (requester)
        .inner_join(users::table.on(
            item_transfers::prev_steward_id.eq(users::id.nullable())  // Join on previous steward (current holder)
        ))
        .inner_join(items::table)
        .order(item_transfers::updated_at.desc())
        .load::<(ItemTransfer, User, Item)>(&mut db)
        .await
        .expect("error getting reserved item transfers");

    // First, find all items stewarded by the current user
    let items_stewarded_by_user: Vec<Uuid> = item_transfers::table
        .filter(item_transfers::status.eq(TransferStatus::Completed))
        .filter(item_transfers::steward_id.eq(user.id))
        .group_by(item_transfers::item_id)
        .select(item_transfers::item_id)
        .load::<Uuid>(&mut db)
        .await
        .expect("error finding stewarded items");

    // Then find all reserved transfers for those items where someone else is requesting them
    let my_outstanding_item_transfers = item_transfers::table
        .filter(item_transfers::status.eq(TransferStatus::Reserved))
        .filter(item_transfers::steward_id.ne(user.id)) // Exclude transfers where user is the steward
        .filter(item_transfers::item_id.eq_any(&items_stewarded_by_user))
        .inner_join(users::table.on(item_transfers::steward_id.eq(users::id))) // Join with users for steward info
        .inner_join(items::table) // Join with items table
        .order(item_transfers::updated_at.desc())
        .load::<(ItemTransfer, User, Item)>(&mut db)
        .await
        .expect("error getting outstanding item transfers");

    let context = context! {
        user,
        my_reserved_item_transfers,
        my_outstanding_item_transfers
    };
    Template::render("item_transfers", &context)
}

#[derive(Debug, PartialEq)]
pub enum TransferRole {
    NewSteward,
    PrevSteward,
}

impl FromStr for TransferRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NewSteward" => Ok(TransferRole::NewSteward),
            "PrevSteward" => Ok(TransferRole::PrevSteward),
            _ => Err(()),
        }
    }
}

impl<'r> FromParam<'r> for TransferRole {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match TransferRole::from_str(param) {
            Ok(tr) => Ok(tr),
            Err(_) => Err(param),
        }
    }
}
// #[derive(FromForm, Debug)]
// pub struct TransferConfirmation {
//     role: TransferRole,
// }

#[put("/items/transfers/<transfer_id>/<role>")]
pub async fn item_transfer_put(
    user: User,
    transfer_id: Uuid,
    role: TransferRole,
    mut db: Connection<Db>
) -> Flash<Redirect> {
    
    // update the item transfer with the appropriate confirmation
    let confirm_time = Utc::now().naive_utc();
    match role {
        TransferRole::NewSteward => {
            diesel::update(item_transfers::table.filter(item_transfers::id.eq(transfer_id)))
                .set(item_transfers::steward_confirmed_at.eq(confirm_time.clone()))
                .get_result::<ItemTransfer>(&mut db)
                .await
                .expect("Could not confirm ItemTransfer");
        },
        TransferRole::PrevSteward => {
            diesel::update(item_transfers::table.filter(item_transfers::id.eq(transfer_id)))
                .set((
                    item_transfers::prev_steward_confirmed_at.eq(confirm_time.clone()),
                    item_transfers::prev_steward_id.eq(Some(user.id))
                ))
                .get_result::<ItemTransfer>(&mut db)
                .await
                .expect("Could not confirm ItemTransfer");
        },
    }

    // after the transfer has been updated, we need to query it to see if both confirmations have been set. If so, then
    // we can update the status of this item transfer to complete.
    let item_transfer = item_transfers::table
        .find(transfer_id)
        .first::<ItemTransfer>(&mut db)
        .await
        .expect("Could not find the item transfer we just updated");

    if item_transfer.steward_confirmed_at.is_some() && item_transfer.prev_steward_confirmed_at.is_some() {
        diesel::update(item_transfers::table.filter(item_transfers::id.eq(transfer_id)))
                .set(item_transfers::status.eq(TransferStatus::Completed))
                .get_result::<ItemTransfer>(&mut db)
                .await
                .expect("Could not complete ItemTransfer");

        //TODO update all OTHER reserved item transfers so that they point
        // at the correct previous steward.
    }

    Flash::success(Redirect::to("/items/transfers"), "Item transfer confirmed! You may have to wait for the other party to confirm before the transfer shows as completed.")
}