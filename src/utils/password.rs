use argon2::{self, Config};
use rand::Rng;

/// Generate a cryptographically secure random salt
fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill(&mut salt[..]);
    salt
}

/// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = generate_salt();
    let config = Config::default();
    
    // Use the default configuration (which is Argon2id)
    let hash = argon2::hash_encoded(
        password.as_bytes(),
        &salt,
        &config
    )?;
    
    Ok(hash)
}

/// Verify a password against a stored hash
pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}
