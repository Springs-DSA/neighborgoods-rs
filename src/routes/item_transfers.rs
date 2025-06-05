use std::str::FromStr;

use chrono::Utc;
use rocket::{form::Form, response::{Flash, Redirect}};
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;
use rocket::request::FromParam;

use crate::{db::Db, models::{item::Item, item_transfer::{ItemTransfer, TransferPurpose, TransferStatus}, user::User}, schema::{item_transfers, items, users}, services::{item_transfer_service::{current_steward_get, get_transfers_context, TransferRole}, notification_service}};





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
        Ok(_) => {
            // Create notification for the current steward (owner) about the borrow request
            let _ = notification_service::create_borrow_request_notification(
                current_transfer.1.id, // owner_id
                &user.name, // borrower_name
                &current_transfer.2.name, // item_name
                item_id,
                &mut db,
            ).await;
            
            Flash::success(Redirect::to(item_url), "Item transfer requested!")
        },
        Err(_) => Flash::error(Redirect::to(item_url), "Item transfer was unable to be created"),
    }
}

#[post("/items/<item_id>/return")]
pub async fn item_return_post(user: User, item_id: Uuid, mut db: Connection<Db>) -> Flash<Redirect> {
    // Get the current steward transfer to find who the item should be returned to
    let current_transfer = current_steward_get(item_id, &mut db)
        .await
        .expect("Unable to get current steward transfer");

    // Verify that the user is the current steward (borrower)
    if current_transfer.1.id != user.id {
        return Flash::error(Redirect::to(format!("/items/{}", item_id)), "You are not the current steward of this item");
    }

    // Check if there's already a pending return request for this item
    let existing_return = item_transfers::table
        .filter(item_transfers::item_id.eq(item_id))
        .filter(item_transfers::status.eq(TransferStatus::Reserved))
        .filter(item_transfers::purpose.eq(TransferPurpose::Return))
        .filter(item_transfers::prev_steward_id.eq(user.id))
        .first::<ItemTransfer>(&mut db)
        .await;

    if existing_return.is_ok() {
        return Flash::error(Redirect::to(format!("/items/{}", item_id)), "A return request for this item is already pending");
    }

    // Find the previous steward (owner/lender) to return the item to
    let owner_id = current_transfer.0.prev_steward_id
        .expect("No previous steward found - cannot determine who to return item to");

    let now = Utc::now().naive_utc();
    // Create the return transfer with the owner as the new steward
    let return_transfer = ItemTransfer {
        id: Uuid::new_v4(),
        item_id,
        steward_id: owner_id,  // Owner becomes the new steward
        prev_steward_id: Some(user.id),  // Current user (borrower) is the previous steward
        purpose: TransferPurpose::Return,
        lat: None,
        lng: None,
        status: TransferStatus::Reserved,
        created_at: now.clone(),
        updated_at: now.clone(),
        steward_confirmed_at: None,
        prev_steward_confirmed_at: None
    };

    let transfer_result = diesel::insert_into(item_transfers::table)
        .values(&return_transfer)
        .execute(&mut db)
        .await;

    let item_url = format!("/items/{}", item_id);

    match transfer_result {
        Ok(_) => {
            // Create notification for the owner about the return request
            let _ = notification_service::create_return_request_notification(
                owner_id, // owner_id
                &user.name, // borrower_name
                &current_transfer.2.name, // item_name
                item_id,
                &mut db,
            ).await;
            
            Flash::success(Redirect::to(item_url), "Item return initiated! Both parties must confirm the return.")
        },
        Err(_) => Flash::error(Redirect::to(item_url), "Item return could not be initiated"),
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

        // Get item information for notifications
        let item = items::table
            .find(item_transfer.item_id)
            .first::<Item>(&mut db)
            .await
            .expect("Could not find item");

        // Create appropriate notifications based on transfer type
        if item_transfer.purpose == TransferPurpose::Return {
            // Return completed - notify the borrower
            if let Some(prev_steward_id) = item_transfer.prev_steward_id {
                let _ = notification_service::create_return_confirmed_notification(
                    prev_steward_id, // borrower_id
                    &item.name, // item_name
                    item_transfer.item_id,
                    &mut db,
                ).await;
            }
        } else {
            // Borrow approved - notify the borrower
            let _ = notification_service::create_borrow_approved_notification(
                item_transfer.steward_id, // borrower_id
                &item.name, // item_name
                item_transfer.item_id,
                &mut db,
            ).await;
        }

        //TODO update all OTHER reserved item transfers so that they point
        // at the correct previous steward.
    }

    Flash::success(Redirect::to("/items/transfers"), "Item transfer confirmed! You may have to wait for the other party to confirm before the transfer shows as completed.")
}

#[post("/items/transfer/<transfer_id>/cancel")]
pub async fn item_transfer_cancel(
    user: User,
    transfer_id: Uuid,
    mut db: Connection<Db>
) -> Flash<Redirect> {
    // Get the transfer to verify the user can cancel it
    let transfer = item_transfers::table
        .find(transfer_id)
        .first::<ItemTransfer>(&mut db)
        .await;

    match transfer {
        Ok(transfer) => {
            // Only allow the requester (steward) to cancel their own request
            if transfer.steward_id != user.id {
                return Flash::error(Redirect::to("/items/transfers"), "You can only cancel your own transfer requests");
            }

            // Only allow canceling if the transfer is still reserved
            if transfer.status != TransferStatus::Reserved {
                return Flash::error(Redirect::to("/items/transfers"), "This transfer cannot be canceled");
            }

            // Update the transfer status to canceled
            let cancel_result = diesel::update(item_transfers::table.filter(item_transfers::id.eq(transfer_id)))
                .set(item_transfers::status.eq(TransferStatus::Canceled))
                .execute(&mut db)
                .await;

            match cancel_result {
                Ok(_) => Flash::success(Redirect::to("/items/transfers"), "Transfer request canceled successfully"),
                Err(_) => Flash::error(Redirect::to("/items/transfers"), "Failed to cancel transfer request"),
            }
        },
        Err(_) => Flash::error(Redirect::to("/items/transfers"), "Transfer not found"),
    }
}
