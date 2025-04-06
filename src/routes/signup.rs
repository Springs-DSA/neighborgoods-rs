// /* To be able to return Templates */
// use std::collections::HashMap;

// /* Diesel query builder */
// use diesel::prelude::*;

// /* Database macros */
// use crate::schema::*;

// /* Database data structs (Hero, NewHero) */
// use crate::models::*;
// use crate::utils;

// /* To be able to parse raw forms */
// use rocket::http::ContentType;
// use rocket::Data;
// use rocket::forms::Form;
// use rocket_multipart_form_data::{
//     MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
// };

// use rocket_db_pools::{Database, Connection};
// use rocket_db_pools::diesel::{PgPool, prelude::*};
// use rocket_dyn_templates::{Template, context};

// /* Flash message and redirect */
// use rocket::request::FlashMessage;
// use rocket::response::{Flash, Redirect};


// #[get("/signup")]
// pub fn signup() -> Template {
//     let mut context = HashMap::new();
//     context.insert("title", "Sign Up");
//     Template::render("signup", &context)
// }

// #[post("/signup", data = "<user_data>")]
// pub fn signup_post(
//     user_data: Data,
//     content_type: &ContentType,
//     flash: Option<FlashMessage>,
//     &mut db: Connection<Db>,
// ) -> Result<Flash<Redirect>, Flash<Redirect>> {
//     // Parse the multipart form data
//     let mut options = MultipartFormDataOptions::new();

//     options.allowed_fields = vec![
//         MultipartFormDataField::text("username"),
//         MultipartFormDataField::text("password"),
//         MultipartFormDataField::text("email"),
//     ];

//     let mut form_data = MultipartFormData::parse(content_type, user_data, options)
//         .map_err(|_| Flash::error(Redirect::to("/signup"), "Failed to parse form data"))?;

//     // Extract the fields from the parsed form data
//     let username = form_data
//         .fields
//         .remove("username")
//         .ok_or_else(|| Flash::error(Redirect::to("/signup"), "Missing username"))?;
//     let password = form_data
//         .fields
//         .remove("password")
//         .ok_or_else(|| Flash::error(Redirect::to("/signup"), "Missing password"))?;
//     let email = form_data
//         .fields
//         .remove("email")
//         .ok_or_else(|| Flash::error(Redirect::to("/signup"), "Missing email"))?;

//     // Hash the password and store the user in the database
//     let hashed_password = utils::password::hash_password(password);
//     match hashed_password {
//         Ok(hashed) => {
//             // Create a new user
//             let new_user = NewUser {
//                 username: username[0].text.clone(),
//                 password: hashed,
//                 email: email[0].text.clone(),
//             };

//             // Insert the new user into the database
//             diesel::insert_into(users::table)
//                 .values(&new_user)
//                 .execute(db)
//                 .map_err(|_| Flash::error(Redirect::to("/signup"), "Failed to create user"))?;
//         }
//         Err(_) => {
//             return Err(Flash::error(
//                 Redirect::to("/signup"),
//                 "Failed to hash password",
//             ));
//         }
//     }

//     // Redirect to a success page or login page
//     Ok(Flash::success(
//         Redirect::to("/"),
//         "Signup successful, user approval pending!",
//     ))
// }
