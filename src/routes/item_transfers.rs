use chrono::Utc;
use rocket::{form::Form, response::{Flash, Redirect}};
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::Template;
use uuid::Uuid;

use crate::{db::Db, models::{item_transfer::{ItemTransfer, TransferPurpose, TransferStatus}, user::User}, schema::item_transfers, services::item_transfer_service::{current_steward_get, get_transfers_context, TransferRole}};





#[derive(FromForm, Debug)]
pub struct TransferRequest {
    purpose: TransferPurpose,
}

#[post("/items/<item_id>/transfer", data = "<transfer_request>")]
pub async fn item_transfer_post(user: User, item_id: Uuid, transfer_request: Form<TransferRequest>, mut db: Connection<Db>) -> Flash<Redirect> {
    // when a transfer is first made, its status will be reserved, and the most recent completed item transfer/user will need to be 
    // found to get the current steward, so they can be marked as the previous steward in the new item transfer.

    let current_transfer = current_steward_get(item_id, &mut db)
        .await
        .expect("Unable to get current steward transfer");

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

    let context = get_transfers_context(user, &mut db).await;
    Template::render("item_transfers", &context)
}

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