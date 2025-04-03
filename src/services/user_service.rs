use uuid::Uuid;
use chrono::Utc;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use rocket_db_pools::diesel::PgConnection;
use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::utils::password;
use crate::models::user::User;
use crate::schema::users;

/// Data Transfer Object for user signup request
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub lat: Option<BigDecimal>,
    pub lng: Option<BigDecimal>,
    pub password: String,
}

/// Response returned from the signup service
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupResponse {
    pub success: bool,
    pub message: String,
    pub is_new_user: bool,
}

/// Creates a new user or updates an existing one with provided signup information
pub fn signup(conn: &mut PgConnection, signup_data: SignupRequest) -> Result<SignupResponse> {
    // Check if user with this email already exists
    let existing_user = users::table
        .filter(users::email.eq(&signup_data.email))
        .first::<User>(conn)
        .optional()?;
    
    let is_new_user = existing_user.is_none();
    
    if let Some(user) = existing_user {
        // Update existing user
        let hash = password::hash_password(&signup_data.password)
            .map_err(|e| anyhow::anyhow!("Password hashing error: {}", e))?;
        
        diesel::update(users::table.find(user.id))
            .set((
                users::name.eq(&signup_data.name),
                users::phone.eq(&signup_data.phone),
                users::lat.eq(&signup_data.lat),
                users::lng.eq(&signup_data.lng),
                users::password_hash.eq(hash),
                users::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        
        Ok(SignupResponse {
            success: true,
            message: "User information updated. Your account is still pending approval.".to_string(),
            is_new_user: false,
        })
    } else {
        // Create new user
        let hash = password::hash_password(&signup_data.password)
            .map_err(|e| anyhow::anyhow!("Password hashing error: {}", e))?;
        let now = Utc::now().naive_utc();
        
        let new_user = User {
            id: Uuid::new_v4(),
            name: signup_data.name,
            email: signup_data.email,
            phone: signup_data.phone,
            lat: signup_data.lat,
            lng: signup_data.lng,
            home_node_id: None,
            password_hash: hash,
            password_reset_token: None,
            password_reset_expiration: None,
            approved_at: None,
            approved_by: None,
            created_at: now,
            updated_at: now,
        };
        
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
        
        Ok(SignupResponse {
            success: true,
            message: "Thank you for signing up! Your account is pending approval.".to_string(),
            is_new_user: true,
        })
    }
}
