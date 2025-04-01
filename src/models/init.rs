use rocket_db_pools::diesel::{prelude::*, AsyncConnection};
use std::env;
use crate::schema::node_settings;

/// Initializes node settings from environment variables if they don't already exist
pub async fn initialize_node_settings<C>(db: &mut C) -> Result<(), diesel::result::Error> 
where
    C: AsyncConnection<Backend = diesel::pg::Pg> + Send,
{
    // Check if node settings already exist
    let count = node_settings::table
        .count()
        .get_result::<i64>(db)
        .await?;

    // Only insert settings if none exist
    if count == 0 {
        // Read environment variables for node-specific settings
        let node_id = env::var("NODE_ID").expect("NODE_ID must be set in the environment variables");
        let node_name = env::var("NODE_NAME").unwrap_or_else(|_| "NeighborGoods Local Node".to_string());
        let node_description = env::var("NODE_DESCRIPTION")
            .unwrap_or_else(|_| "A local instance of the NeighborGoods federated library of things".to_string());
        let node_w3w = env::var("NODE_W3W").unwrap_or_else(|_| "".to_string());

        // Settings to insert
        let settings = vec![
            // Node settings from environment variables
            (node_id.clone(), String::from("name"), node_name, String::from("string")),
            (node_id.clone(), String::from("description"), node_description, String::from("string")),
            (node_id.clone(), String::from("w3w"), node_w3w, String::from("string")),
            
            // Hard-coded default settings
            (node_id.clone(), String::from("item_budget_per_person"), String::from("5"), String::from("integer")),
            (node_id.clone(), String::from("ca_min_votes"), String::from("3"), String::from("integer")),
            (node_id.clone(), String::from("ca_expiry_days"), String::from("14"), String::from("integer")),
        ];

        // Insert all settings
        for (entity, attribute, value, data_type) in settings {
            diesel::insert_into(node_settings::table)
                .values((
                    node_settings::entity.eq(entity),
                    node_settings::attribute.eq(attribute),
                    node_settings::value.eq(value),
                    node_settings::data_type.eq(data_type),
                ))
                .execute(db)
                .await?;
        }
    }

    Ok(())
}
