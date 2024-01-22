use std::fmt;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use super::jwt_struct::Claims;

#[derive(Debug)]
pub enum ValidationError {
    Jwt(jsonwebtoken::errors::Error),
    MissingBearer,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::Jwt(e) => write!(f, "JWT error: {}", e),
            ValidationError::MissingBearer => write!(f, "Missing 'Bearer' prefix"),
        }
    }
}

impl std::error::Error for ValidationError {}

pub fn create_jwt(
    secret_key: &[u8],
    id: String,
    username: String,
) -> Result<String, ValidationError> {
    let claims = Claims {
        id,
        username,
        exp: (jsonwebtoken::get_current_timestamp() + 3600) as usize,
        iat: jsonwebtoken::get_current_timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
    .map_err(ValidationError::Jwt)
}

pub fn validate_jwt(secret_key: &[u8], token: &str) -> Result<Claims, ValidationError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
    .map_err(ValidationError::Jwt)
}

pub fn validate_authorization(
    secret_key: &[u8],
    authorization: &str,
) -> Result<Claims, ValidationError> {
    authorization
        .strip_prefix("Bearer")
        .ok_or(ValidationError::MissingBearer)
        .and_then(|token| validate_jwt(secret_key, token))
}
