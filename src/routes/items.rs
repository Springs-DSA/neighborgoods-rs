use std::fs;
use std::ops::DerefMut;

use chrono::Utc;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::{fs::TempFile, response::{Flash, Redirect}};
use uuid::Uuid;
use crate::models::item_transfer::{TransferPurpose, TransferStatus};
use crate::services::item_service::get_item_view_context;
use crate::{db::Db, models::{item::Item, item_transfer::ItemTransfer, user::User}, schema::{item_transfers, items, users}};

#[get("/inventory")]
pub async fn inventory_get(user: User, mut db: Connection<Db>) -> Template {
    // get list of items
    let items = items::table
        .inner_join(users::table)
        .load::<(Item, User)>(&mut db)
        .await
        .expect("Error loading items");
    let context = context! {
        items,
        user
    };
    Template::render("inventory", &context)
}

#[derive(FromForm, Debug)]
pub struct ItemData<'r> {
    r#name: &'r str,
    r#description: &'r str,
    r#category: &'r str,
    image: TempFile<'r>,
}

#[get("/items/contribute")]
pub async fn items_contribute_get(user: User) -> Template {
    let context = context! {
        user
    };
    Template::render("item_contribution", &context)
}

#[post("/items/contribute", data = "<item>")]
pub async fn items_contribute_post(user: User, mut item: Form<ItemData<'_>>, mut db: Connection<Db>) -> Flash<Redirect> {
    println!("ItemData: {:?}", item);
    // create the item in the database
    let item_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();
    let mut new_item = Item {
        id: item_id,
        name: item.name.into(),
        description: Some(item.description.into()),
        contributed_by: user.id,
        upload_directory_path: format!("public/{}/", item_id.to_string()),
        created_at: now.clone(),
        updated_at: now.clone()
    };

    // create item upload dir.
    let dir_path = format!("uploads/{}", item_id.to_string());
    fs::create_dir(dir_path).unwrap();

    // upload item
    if let Some(name) = item.deref_mut().image.name() {

        let path = format!("uploads/{}/{}", item_id.to_string(), name);
        // attach the image file to the end of the upload_directory_path
        // TODO need to be able to handle all the files in the directory, not just one
        new_item.upload_directory_path.push_str(name);
        
        let upload_result = item.image.move_copy_to(path).await;
        match upload_result {
            Ok(_) => {
                println!(">>> file path: {}", item.image.path().unwrap().to_str().unwrap());
                println!(">>> item upload dir path: {}", new_item.upload_directory_path);
            },
            Err(e) => {
                println!("ERROR: {}", e); // need to have full path specified.
                return Flash::error(Redirect::to("/inventory"), "Item file(s) couldn't be uploaded");
            }
        }
    }

    // if we've arrived here, then we have successfully uploaded.

    let db_result = diesel::insert_into(items::table)
        .values(&new_item)
        .execute(&mut db)
        .await;
    
    match db_result {
        Ok(_) => {
            // also, create the first item transfer to move the item to the contributing user's stewardship.

            let initial_item_transfer = ItemTransfer { 
                id: Uuid::new_v4(),
                item_id,
                steward_id: user.id,
                prev_steward_id: None,
                purpose: TransferPurpose::Contribute,
                lat: None,
                lng: None,
                status: TransferStatus::Completed,
                created_at: now.clone(),
                updated_at: now.clone(),
                steward_confirmed_at: Some(now.clone()),
                prev_steward_confirmed_at: Some(now.clone())
            };

            let iit_result = diesel::insert_into(item_transfers::table)
                .values(&initial_item_transfer)
                .execute(&mut db)
                .await;
            match iit_result {
                Ok(_) => {
                    println!("Successfully created item transfer");
                    Flash::success(Redirect::to("/inventory"), "Item successfully created")
                },
                Err(e) => {
                    // TODO delete item record, remove uploads.
                    // https://stackoverflow.com/questions/75939019/transactions-in-rust-diesel
                    println!(">>> couldn't create item transfer, {}", e);
                    Flash::error(Redirect::to("/inventory"), "Item record couldn't be created")
                }
            
            }

            
        },
        Err(e) => {
            println!("ERROR: {}", e);
            //TODO remove uploaded folder/files
            Flash::error(Redirect::to("/inventory"), "Item record couldn't be created")
        },
    }
}

// get an individual item's page.
#[get("/items/<item_id>")]
pub async fn item_get(user: User, item_id: Uuid, db: Connection<Db>) -> Template {
    let context_result = get_item_view_context(user, item_id, db).await;
    let context = match context_result {
        Ok(context) => context,
        Err(_) => return Template::render("error", context! { error: "Could not make context" }),
    };
    
    Template::render("item_view", context)
}

#[delete("/items/<item_id>")]
pub async fn item_delete(user: User, item_id: Uuid, mut db: Connection<Db>) -> Flash<Redirect> {
    // full delete an item. This deletes it from the database, removes uploads, and removes item transfers

    // first, check if this user is the one who contributed the item
    let item = items::table
        .find(item_id)
        .first::<Item>(&mut db)
        .await
        .expect("unable to find item");

    if item.contributed_by == user.id {
        // delete item, should cascade through item transfers
        diesel::delete(
            items::table
            .filter(items::id.eq(item_id))
        ).execute(&mut db)
        .await
        .expect("Couldn't delete item");
        
        Flash::success(Redirect::to("/inventory"), "Item deleted!")
    } else {
        Flash::error(Redirect::to(format!("/items/{}", item_id)), "Item record can't be deleted by non-owner")
    }

}