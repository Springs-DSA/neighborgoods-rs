use crate::db::Db;
use crate::models::user::User;
use crate::services::notification_service;
use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct NotificationResponse {
    pub success: bool,
    pub message: String,
}

#[get("/notifications")]
pub async fn get_notifications(
    user: User,
    mut db: Connection<Db>,
) -> Result<Json<Vec<crate::models::notification::Notification>>, rocket::response::status::BadRequest<String>> {
    match notification_service::get_user_notifications(user.id, &mut db).await {
        Ok(notifications) => Ok(Json(notifications)),
        Err(e) => Err(rocket::response::status::BadRequest(format!("Failed to fetch notifications: {}", e))),
    }
}

#[post("/notifications/<notification_id>/read")]
pub async fn mark_notification_read(
    notification_id: Uuid,
    user: User,
    mut db: Connection<Db>,
) -> Result<Json<NotificationResponse>, rocket::response::status::BadRequest<String>> {
    match notification_service::mark_notification_as_read(notification_id, user.id, &mut db).await {
        Ok(_) => Ok(Json(NotificationResponse {
            success: true,
            message: "Notification marked as read".to_string(),
        })),
        Err(e) => Err(rocket::response::status::BadRequest(format!("Failed to mark notification as read: {}", e))),
    }
}

#[get("/notifications/unread-count")]
pub async fn get_unread_count(
    user: User,
    mut db: Connection<Db>,
) -> Result<Json<i64>, rocket::response::status::BadRequest<String>> {
    match notification_service::get_unread_count(user.id, &mut db).await {
        Ok(count) => Ok(Json(count)),
        Err(e) => Err(rocket::response::status::BadRequest(format!("Failed to get unread count: {}", e))),
    }
}

#[post("/notifications/test")]
pub async fn create_test_notification(
    user: User,
    mut db: Connection<Db>,
) -> Result<Json<NotificationResponse>, rocket::response::status::BadRequest<String>> {
    match notification_service::create_notification(
        user.id,
        "Test Notification".to_string(),
        "This is a test notification to verify the system is working.".to_string(),
        Some("/dashboard".to_string()),
        "test".to_string(),
        &mut db,
    ).await {
        Ok(_) => Ok(Json(NotificationResponse {
            success: true,
            message: "Test notification created".to_string(),
        })),
        Err(e) => Err(rocket::response::status::BadRequest(format!("Failed to create test notification: {}", e))),
    }
}

pub fn routes() -> Vec<Route> {
    routes![get_notifications, mark_notification_read, get_unread_count, create_test_notification]
}
