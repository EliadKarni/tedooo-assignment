use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const SECRET: &[u8] = b"your_secret_key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// Generates a JWT for a given user ID.
///
/// # Arguments
/// - `user_id`: The user ID to include in the token.
///
/// # Returns
/// A signed JWT as a `String`.
pub fn generate_jwt(user_id: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // Token valid for 1 hour

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

/// Validates a JWT and returns the claims if valid.
///
/// # Arguments
/// - `token`: The JWT to validate.
///
/// # Returns
/// The decoded claims if the token is valid.
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
}