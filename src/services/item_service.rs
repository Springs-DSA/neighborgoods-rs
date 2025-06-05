use anyhow::Result;

use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use crate::db::Db;
use crate::models::item::Item;
use crate::models::user::User;
use crate::models::item_transfer::{ItemTransfer, TransferStatus, TransferPurpose};
use crate::schema::{item_transfers, items, users};

#[derive(Serialize)]
pub struct ItemViewContext {
    pub user: User,
    pub item: Item,
    pub contributor: User,
    pub transfers_with_stewards: Vec<(ItemTransfer, User)>,
    pub current_steward: User,
    pub is_current_steward: bool,
    pub has_pending_return: bool
}

pub async fn get_item_view_context(user: User, item_id: Uuid, mut db: Connection<Db>) -> Result<ItemViewContext> {
    // 1. Load just the item first
    let item = items::table
        .find(item_id)
        .get_result::<Item>(&mut db)
        .await?;
    
    // 2. Load the contributor
    let contributor = users::table
        .find(item.contributed_by)
        .get_result::<User>(&mut db)
        .await?;
    
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
    let current_steward = item_transfers::table
        .filter(item_transfers::item_id.eq(item_id))
        .filter(item_transfers::status.eq(TransferStatus::Completed))
        .inner_join(users::table.on(item_transfers::steward_id.eq(users::id)))
        .order(item_transfers::updated_at.desc())
        .first::<(ItemTransfer, User)>(&mut db)
        .await
        .expect("error getting item transfers")
        .1;
    
    // 5. Check if the current user is the steward
    let is_current_steward = current_steward.id == user.id;
    
    // 6. Check if there's a pending return request for this item
    let has_pending_return = item_transfers::table
        .filter(item_transfers::item_id.eq(item_id))
        .filter(item_transfers::status.eq(TransferStatus::Reserved))
        .filter(item_transfers::purpose.eq(TransferPurpose::Return))
        .first::<ItemTransfer>(&mut db)
        .await
        .is_ok();
    
    Ok(ItemViewContext {
        user,
        item,
        contributor,
        transfers_with_stewards,
        current_steward,
        is_current_steward,
        has_pending_return
    })
}
