use rocket::{form::Form, response::{Flash, Redirect}, post, delete};
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::{db::Db, models::user::User, services::tag_service};

// Form struct for tag creation
#[derive(FromForm, Debug)]
pub struct TagForm<'r> {
    name: &'r str,
}

// Form struct for tag assignment
#[derive(FromForm, Debug)]
pub struct ItemTagForm<'r> {
    tag_name: &'r str,
}

// POST route for creating new tags
#[post("/tags", data = "<tag_form>")]
pub async fn tag_create(user: User, tag_form: Form<TagForm<'_>>, db: Connection<Db>) -> Flash<Redirect> {
    // Use tag_service.tag_create to create the tag
    match tag_service::tag_create(tag_form.name, db).await {
        Ok(_) => Flash::success(Redirect::to("/inventory"), "Tag created successfully"),
        Err(_) => Flash::error(Redirect::to("/inventory"), "Failed to create tag")
    }
}

// POST route for assigning tags to items
#[post("/items/<item_id>/tags", data = "<item_tag_form>")]
pub async fn item_tag_assign(
    user: User,
    item_id: Uuid, 
    item_tag_form: Form<ItemTagForm<'_>>, 
    db: Connection<Db>
) -> Flash<Redirect> {
    // Use tag_service.item_tag_assign to assign the tag to the item
    match tag_service::item_tag_assign(item_id, item_tag_form.tag_name, db).await {
        Ok(_) => Flash::success(Redirect::to(format!("/items/{}", item_id)), "Tag assigned successfully"),
        Err(_) => Flash::error(Redirect::to(format!("/items/{}", item_id)), "Failed to assign tag")
    }
}

// DELETE route for revoking tag assignments
#[delete("/items/<item_id>/tags/<tag_name>")]
pub async fn item_tag_revoke(
    user: User,
    item_id: Uuid, 
    tag_name: String, 
    db: Connection<Db>
) -> Flash<Redirect> {
    // Use tag_service.item_tag_revoke to revoke the tag from the item
    match tag_service::item_tag_revoke(item_id, &tag_name, db).await {
        Ok(true) => Flash::success(Redirect::to(format!("/items/{}", item_id)), "Tag removed successfully"),
        Ok(false) => Flash::error(Redirect::to(format!("/items/{}", item_id)), "Tag was not assigned to this item"),
        Err(_) => Flash::error(Redirect::to(format!("/items/{}", item_id)), "Failed to remove tag")
    }
}
