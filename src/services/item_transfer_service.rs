use anyhow::Result;
// this module provides functions for transferring stewardship of items and calculating related data.
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use crate::db::Db;
use crate::models::item::Item;
use crate::models::user::User;
use crate::models::item_transfer::{ItemTransfer, TransferStatus};
use crate::schema::{item_transfers, items, users};



pub async fn current_steward_get(item_id: Uuid, db: &mut Connection<Db>) -> Result<(ItemTransfer, User, Item), diesel::result::Error> {
    // get the item transfer and user
    item_transfers::table
        .filter(item_transfers::item_id.eq(item_id))
        .filter(item_transfers::status.eq(TransferStatus::Completed))
        .inner_join(users::table.on(item_transfers::steward_id.eq(users::id)))
        .inner_join(items::table.on(items::id.eq(item_transfers::item_id)))
        .order(item_transfers::updated_at.desc())
        .first::<(ItemTransfer, User, Item)>(db)
        .await
}

