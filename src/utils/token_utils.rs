use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Validation, DecodingKey, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub userid: String,  
    pub company_id: String,
    pub role: String, 
    pub exp: usize, 
    pub iat: usize,
}


pub fn generate_token(user_id: &str, role_id: &str, company_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        userid: user_id.to_owned(),
        company_id: company_id.to_owned(),
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