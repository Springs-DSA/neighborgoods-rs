use rocket::request::{self, Request, Outcome, FromRequest};
use uuid::Uuid;
use crate::models::user::User;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use crate::schema::users;
use rocket::http::Status;
use crate::Db;


#[rocket::async_trait]
impl <'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, ()> {
        // Get database connection
        let db_outcome = request.guard::<Connection<Db>>().await;
        let mut db = match db_outcome {
            Outcome::Success(db) => db,
            _ => return Outcome::Forward(Status::Unauthorized),
        };

        // Get user ID from cookie
        let user_id = match request.cookies()
            .get_private("user_id")
            .and_then(|cookie| {
                let val =cookie.value();
                Uuid::parse_str(val).ok()
            }) {
                Some(id) => id,
                None => return Outcome::Forward(Status::Unauthorized),
            };

        
        // Query database with the user ID
        match users::table
            .find(user_id)
            .first::<User>(&mut db)
            .await {
                Ok(user) => Outcome::Success(user),
                Err(_) => Outcome::Forward(Status::Unauthorized),
            }
    }
}