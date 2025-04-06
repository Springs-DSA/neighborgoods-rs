use std::env;

use chrono::Utc;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::response::{Flash, Redirect};
use crate::models::user::User;
use crate::utils::password;
use rocket::form::Form;
use crate::schema::users;

use crate::Db;

#[derive(FromForm)]
pub struct UserData<'r> {
    r#username: &'r str,
    r#email: &'r str,
    r#password: &'r str,
    r#password_confirm: &'r str
}

#[get("/signup")]
pub fn signup_get() -> Template {
    let context = context! {};
    Template::render("signup", &context)
}

#[post("/signup", data = "<user>")]
pub async fn signup_post(user: Form<UserData<'_>>, mut db: Connection<Db>) -> Flash<Redirect> {

    // hash passwords
    if user.password != user.password_confirm {
        return Flash::error(Redirect::to("/signup"), "Passwords do not match!")
    } 

    let hashed_password = password::hash_password(user.password);
    match hashed_password {
        Ok(hp) => {

            let now = Utc::now().naive_utc();

            // Insert the new user into the database
            let new_user = User {
                id: uuid::Uuid::new_v4(),
                name: user.username.into(),
                email: user.email.into(),
                password_hash: hp.clone(),
                phone: None,
                lat: None,
                lng: None,
                home_node_id: Some(env::var("NODE_ID").expect("NODE_ID must be set in the environment variables")),
                password_reset_token: None,
                password_reset_expiration: None,
                created_at: now.clone(),
                updated_at: now.clone(),
                approved_at: None,
                approved_by: None

            };
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut db)
                .await
                .expect("Failed to create user");
        }
        Err(_) => {
            return Flash::error(Redirect::to("/signup"), "Password Hash failed");
        }
    }


    Flash::success(Redirect::to("/"), "User created! Approval pending...")
}