use crate::db::Db;
use crate::models::notification::{Notification, NewNotification};
use crate::models::user::User;
use crate::schema::notifications;
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use uuid::Uuid;

pub async fn create_notification(
    user_id: Uuid,
    title: String,
    body: String,
    link: Option<String>,
    notification_type: String,
    db: &mut Connection<Db>,
) -> Result<Notification, diesel::result::Error> {
    let new_notification = NewNotification::new(user_id, title, body, link, notification_type);
    
    diesel::insert_into(notifications::table)
        .values(&new_notification)
        .get_result(db)
        .await
}

pub async fn get_user_notifications(
    user_id: Uuid,
    db: &mut Connection<Db>,
) -> Result<Vec<Notification>, diesel::result::Error> {
    notifications::table
        .filter(notifications::user_id.eq(user_id))
        .order(notifications::created_at.desc())
        .limit(20) // Limit to 20 most recent notifications
        .load::<Notification>(db)
        .await
}

pub async fn mark_notification_as_read(
    notification_id: Uuid,
    user_id: Uuid,
    db: &mut Connection<Db>,
) -> Result<Notification, diesel::result::Error> {
    diesel::update(
        notifications::table
            .filter(notifications::id.eq(notification_id))
            .filter(notifications::user_id.eq(user_id))
    )
    .set(notifications::read.eq(true))
    .get_result(db)
    .await
}

pub async fn get_unread_count(
    user_id: Uuid,
    db: &mut Connection<Db>,
) -> Result<i64, diesel::result::Error> {
    notifications::table
        .filter(notifications::user_id.eq(user_id))
        .filter(notifications::read.eq(false))
        .count()
        .get_result(db)
        .await
}

// Helper functions for creating specific notification types
pub async fn create_borrow_request_notification(
    owner_id: Uuid,
    borrower_name: &str,
    item_name: &str,
    item_id: Uuid,
    db: &mut Connection<Db>,
) -> Result<Notification, diesel::result::Error> {
    create_notification(
        owner_id,
        "New Borrow Request".to_string(),
        format!("{} wants to borrow your {}", borrower_name, item_name),
        Some(format!("/items/{}", item_id)),
        "borrow_request".to_string(),
        db,
    ).await
}

pub async fn create_borrow_approved_notification(
    borrower_id: Uuid,
    item_name: &str,
    item_id: Uuid,
    db: &mut Connection<Db>,
) -> Result<Notification, diesel::result::Error> {
    create_notification(
        borrower_id,
        "Borrow Request Approved".to_string(),
        format!("Your request to borrow {} has been approved", item_name),
        Some(format!("/items/{}", item_id)),
        "borrow_approved".to_string(),
        db,
    ).await
}

pub async fn create_return_request_notification(
    owner_id: Uuid,
    borrower_name: &str,
    item_name: &str,
    item_id: Uuid,
    db: &mut Connection<Db>,
) -> Result<Notification, diesel::result::Error> {
    create_notification(
        owner_id,
        "Item Return".to_string(),
        format!("{} is returning your {}", borrower_name, item_name),
        Some(format!("/items/{}", item_id)),
        "return_request".to_string(),
        db,
    ).await
}

pub async fn create_return_confirmed_notification(
    borrower_id: Uuid,
    item_name: &str,
    item_id: Uuid,
    db: &mut Connection<Db>,
) -> Result<Notification, diesel::result::Error> {
    create_notification(
        borrower_id,
        "Return Confirmed".to_string(),
        format!("Your return of {} has been confirmed", item_name),
        Some(format!("/items/{}", item_id)),
        "return_confirmed".to_string(),
        db,
    ).await
}
