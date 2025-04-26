use anyhow::Result;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use uuid::Uuid;
use crate::db::Db;
use crate::models::tag::Tag;
use crate::models::item_tag::ItemTag;
use crate::models::item::Item;
use crate::schema::{tags, item_tags, items};

// Enum for item_search_by_tags search mode
#[derive(Debug, PartialEq)]
pub enum TagSearchMode {
    All,       // All tags must match
    Any,       // Any tag must match
    NoneOf,    // No tag must match
}

// Helper function to format tag names consistently
fn format_tag_name(name: &str) -> String {
    name.trim()
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// Creates a new tag with standardized format
pub async fn tag_create(name: &str, mut db: Connection<Db>) -> Result<Tag> {
    // Format tag name
    let formatted_name = format_tag_name(name);
    
    // Check if tag already exists
    let existing_tag = tags::table
        .filter(tags::name.eq(&formatted_name))
        .first::<Tag>(&mut db)
        .await
        .optional()?;
    
    // Return existing tag if found
    if let Some(tag) = existing_tag {
        return Ok(tag);
    }
    
    // Create new tag
    let now = chrono::Utc::now().naive_utc();
    let new_tag = Tag {
        name: formatted_name,
        created_at: now,
        updated_at: now,
    };
    
    diesel::insert_into(tags::table)
        .values(&new_tag)
        .execute(&mut db)
        .await?;
    
    Ok(new_tag)
}

// Assigns a tag to an item
pub async fn item_tag_assign(item_id: Uuid, tag_name: &str, mut db: Connection<Db>) -> Result<ItemTag> {
    // Format tag name
    let formatted_name = format_tag_name(tag_name);
    
    // Check if tag exists, if not create it
    let tag = match tags::table
        .filter(tags::name.eq(&formatted_name))
        .first::<Tag>(&mut db)
        .await
        .optional()? {
            Some(tag) => tag,
            None => {
                // Create new tag
                let now = chrono::Utc::now().naive_utc();
                let new_tag = Tag {
                    name: formatted_name.clone(),
                    created_at: now,
                    updated_at: now,
                };
                
                diesel::insert_into(tags::table)
                    .values(&new_tag)
                    .execute(&mut db)
                    .await?;
                
                new_tag
            }
        };
    
    // Check if item-tag association already exists
    let existing_item_tag = item_tags::table
        .filter(item_tags::item_id.eq(item_id))
        .filter(item_tags::tag.eq(&tag.name))
        .first::<ItemTag>(&mut db)
        .await
        .optional()?;
    
    // Return existing item_tag if found
    if let Some(item_tag) = existing_item_tag {
        return Ok(item_tag);
    }
    
    // Create new item_tag
    let now = chrono::Utc::now().naive_utc();
    let new_item_tag = ItemTag {
        item_id,
        tag: tag.name,
        created_at: now,
    };
    
    diesel::insert_into(item_tags::table)
        .values(&new_item_tag)
        .execute(&mut db)
        .await?;
    
    Ok(new_item_tag)
}

// Revokes a tag from an item
pub async fn item_tag_revoke(item_id: Uuid, tag_name: &str, mut db: Connection<Db>) -> Result<bool> {
    // Format tag name for consistency
    let formatted_name = format_tag_name(tag_name);
    
    let rows_deleted = diesel::delete(
        item_tags::table
            .filter(item_tags::item_id.eq(item_id))
            .filter(item_tags::tag.eq(formatted_name))
    )
    .execute(&mut db)
    .await?;
    
    Ok(rows_deleted > 0)
}

// Searches items by tags
pub async fn item_search_by_tags(tag_names: Vec<String>, search_mode: TagSearchMode, mut db: Connection<Db>) -> Result<Vec<Item>> {
    // Format tag names to ensure consistent search
    let formatted_tags: Vec<String> = tag_names.iter()
        .map(|name| format_tag_name(name))
        .collect();
    
    // Build query based on search mode
    let items = match search_mode {
        TagSearchMode::All => {
            // Get items that have ALL the specified tags
            let mut items_with_all_tags = Vec::new();
            
            // First, get all items that have at least one of the tags
            let items_with_any_tag: Vec<Item> = items::table
                .inner_join(item_tags::table.on(items::id.eq(item_tags::item_id)))
                .filter(item_tags::tag.eq_any(&formatted_tags))
                .select(items::all_columns)
                .distinct()
                .load::<Item>(&mut db)
                .await?;
            
            // Then filter to keep only those that have all the tags
            for item in items_with_any_tag {
                let item_tags: Vec<String> = item_tags::table
                    .filter(item_tags::item_id.eq(item.id))
                    .select(item_tags::tag)
                    .load::<String>(&mut db)
                    .await?;
                
                let has_all_tags = formatted_tags.iter().all(|tag| item_tags.contains(tag));
                if has_all_tags {
                    items_with_all_tags.push(item);
                }
            }
            
            items_with_all_tags
        },
        TagSearchMode::Any => {
            // Get items that have ANY of the specified tags
            items::table
                .inner_join(item_tags::table.on(items::id.eq(item_tags::item_id)))
                .filter(item_tags::tag.eq_any(&formatted_tags))
                .select(items::all_columns)
                .distinct()
                .load::<Item>(&mut db)
                .await?
        },
        TagSearchMode::NoneOf => {
            // Get items that have NONE of the specified tags
            let items_with_tags: Vec<Uuid> = item_tags::table
                .filter(item_tags::tag.eq_any(&formatted_tags))
                .select(item_tags::item_id)
                .distinct()
                .load::<Uuid>(&mut db)
                .await?;
            
            items::table
                .filter(items::id.ne_all(&items_with_tags))
                .load::<Item>(&mut db)
                .await?
        }
    };
    
    Ok(items)
}