use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Validation, DecodingKey, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  
    pub tenant_id: String,
    pub role: String, 
    pub exp: usize, 
    pub iat: usize,
}


pub fn generate_token(user_id: &str, role_id: &str, tenant_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        tenant_id: tenant_id.to_owned(),
        role: role_id.to_owned(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_token(token: &str) -> Result<Claims, String>{
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|e| e.to_string())?;


    Ok(token_data.claims)
}