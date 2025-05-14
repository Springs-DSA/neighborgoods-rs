
use rocket_dyn_templates::{Template, context};
use rocket::response::{Flash, Redirect};
use crate::models::user::User;


#[get("/profile")]
pub fn profile_get(_user:User) -> Template {
    let context = context! {};
    Template::render("profile", &context)
}
