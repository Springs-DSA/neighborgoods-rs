use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::response::{Flash, Redirect};
use crate::models::user::User;
use crate::utils::password;
use rocket::http::{Cookie, CookieJar};
use rocket::form::Form;
use crate::schema::users;

use crate::Db;


#[derive(FromForm)]
pub struct LoginData<'r> {
    r#email: &'r str,
    r#password: &'r str
}

#[get("/login")]
pub fn login_get() -> Template {
    let context = context! {};
    Template::render("login", &context)
}

#[post("/login", data = "<login>")]
pub async fn login_post(
    login: Form<LoginData<'_>>,
    mut db: Connection<Db>,
    cookies: &CookieJar<'_>
) -> Flash<Redirect> {
    // get the user by email
    let result = users::table
        .filter(users::email.eq(login.email))
        .first::<User>(&mut db)
        .await;

    match result {
        Ok(user) => {
            // if a user is found, check against the hashed password.
            if let Ok(correct_pw) = password::verify_password(&user.password_hash, &login.password) {
                if correct_pw {
                    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                    Flash::success(Redirect::to("/dashboard"), "User Logged In!")
                } else {
                    Flash::error(Redirect::to("/login"), "Incorrect email or password")
                }
            } else {
                Flash::error(Redirect::to("/login"), "Error Verifying Password")
            }

        }
        Err(_) => {
            Flash::error(Redirect::to("/login"), "Incorrect email or password")
        }
    }
}

/// Remove the `user_id` cookie.
#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private("user_id");
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}