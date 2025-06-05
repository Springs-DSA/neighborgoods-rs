use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::response::Redirect;
use crate::{db::Db, models::{item::Item, item_transfer::ItemTransfer, user::User}, schema::{item_transfers, items}};
use crate::models::init::get_node_setting;
use crate::services::notification_service;


#[get("/dashboard")]
pub async fn dashboard_get(user: User, mut db: Connection<Db>) -> Template {
    // get the items user is currently the steward of:
    // items::table.load::<Item>(&mut db).await.expect("Error loading items");
    let items = item_transfers::table
        .inner_join(items::table)
        .filter(item_transfers::steward_id.eq(user.id))
        .load::<(ItemTransfer, Item)>(&mut db)    
        .await
        .expect("Error loading items and transfers");

    let node_name = get_node_setting(&mut db, "name").await.unwrap_or_else(|| "NeighborGoods Local Node".to_string());
    let node_description = get_node_setting(&mut db, "description").await.unwrap_or_else(|| "".to_string());

    // Get user notifications
    let notifications = notification_service::get_user_notifications(user.id, &mut db)
        .await
        .unwrap_or_else(|_| vec![]);

    let context = context! {
        user,
        items,
        node_name,
        node_description,
        notifications
    };
    Template::render("dashboard", &context)
}


#[get("/dashboard", rank = 2)]
pub fn dashboard_redirect() -> Redirect {
    Redirect::to("/")
}
