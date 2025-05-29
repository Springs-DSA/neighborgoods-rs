use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::{fs::TempFile, response::{Flash, Redirect}};

#[get("/user_agreement")]
pub async fn user_agreement_get() -> Template {
    Template::render("user_agreement", &context! {})
}