use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Result};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {  // Tornar a struct Claims pública
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(user_id: &str, secret: &str) -> Result<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600; // Token válido por 1 hora

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {  // Retornar Claims
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).map(|data| data.claims)
}
