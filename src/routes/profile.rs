use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::diesel::prelude::*;
use crate::{db::Db, models::{item::Item, item_transfer::ItemTransfer, user::User}, schema::{item_transfers, items}};
use crate::models::init::get_node_setting;


#[get("/profile")]
pub async fn profile_get(user:User, mut db: Connection<Db>) -> Template {
let items = item_transfers::table
        .inner_join(items::table)
        .filter(item_transfers::steward_id.eq(user.id))
        .load::<(ItemTransfer, Item)>(&mut db)    
        .await
        .expect("Error loading items and transfers");

    let node_name = get_node_setting(&mut db, "name").await.unwrap_or_else(|| "NeighborGoods Local Node".to_string());

    let context = context! {
        user,
        items,
        node_name,};
    Template::render("profile", &context)
}
