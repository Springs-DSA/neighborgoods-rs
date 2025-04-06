use rocket_dyn_templates::{Template, context};
use rocket::response::Redirect;
use crate::models::user::User;

#[get("/dashboard")]
pub fn dashboard_get(user: User) -> Template {
    let context = context! {
        user
    };
    Template::render("dashboard", &context)
}


#[get("/dashboard", rank = 2)]
pub fn dashboard_redirect() -> Redirect {
    Redirect::to("/")
}