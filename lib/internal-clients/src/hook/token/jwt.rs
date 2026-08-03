use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid, // user id
    pub exp: i64,
    pub iat: i64,
    pub ver: String,
}

pub fn verify_access_token(token: &str, secret: &str) -> Result<TokenData<Claims>, JwtError> {
    let mut v = Validation::new(Algorithm::HS256);
    v.leeway = 30;
    v.validate_exp = true;
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &v)
}
