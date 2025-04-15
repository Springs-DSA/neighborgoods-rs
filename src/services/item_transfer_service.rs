use std::str::FromStr;

use anyhow::Result;
use rocket::request::FromParam;
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

#[derive(Serialize)]
pub struct TransfersContext {
    pub user: User,
    pub my_reserved_item_transfers: Vec<(ItemTransfer, User, Item)>,
    pub my_outstanding_item_transfers: Vec<(ItemTransfer, User, Item)>,
}

pub async fn get_transfers_context(user: User, db: &mut Connection<Db>) -> TransfersContext {
    let my_reserved_item_transfers = item_transfers::table
        .filter(item_transfers::status.eq(TransferStatus::Reserved))
        .filter(item_transfers::steward_id.eq(user.id))  // User is the new steward (requester)
        .inner_join(users::table.on(
            item_transfers::prev_steward_id.eq(users::id.nullable())  // Join on previous steward (current holder)
        ))
        .inner_join(items::table)
        .order(item_transfers::updated_at.desc())
        .load::<(ItemTransfer, User, Item)>(db)
        .await
        .expect("error getting reserved item transfers");

    // First, find all items stewarded by the current user
    let items_stewarded_by_user: Vec<Uuid> = item_transfers::table
        .filter(item_transfers::status.eq(TransferStatus::Completed))
        .filter(item_transfers::steward_id.eq(user.id))
        .group_by(item_transfers::item_id)
        .select(item_transfers::item_id)
        .load::<Uuid>(db)
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
        .load::<(ItemTransfer, User, Item)>(db)
        .await
        .expect("error getting outstanding item transfers");

    TransfersContext {
        user,
        my_reserved_item_transfers,
        my_outstanding_item_transfers,
    }
}