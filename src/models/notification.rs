use super::super::schema::notifications;
use rocket_db_pools::diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = notifications)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub link: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub read: bool,
    pub notification_type: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub link: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub read: bool,
    pub notification_type: String,
}

impl NewNotification {
    pub fn new(
        user_id: Uuid,
        title: String,
        body: String,
        link: Option<String>,
        notification_type: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            body,
            link,
            created_at: chrono::Utc::now().naive_utc(),
            read: false,
            notification_type,
        }
    }
}
