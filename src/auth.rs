// src/auth.rs (nouveau module)
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
}

const SECRET: &[u8] = b"my_secret_key_change_in_prod";

pub fn create_token(user_id: &str) -> Result<String> {
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims { sub: user_id.to_string(), exp };
    Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))?)
}

pub fn verify_token(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default())?;
    Ok(token_data.claims)
}
