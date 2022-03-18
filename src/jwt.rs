use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use crate::models::CurrentUser;

const JWT_SECRET: &[u8; 6] = b"secret";

pub fn create_jwt(user: &CurrentUser) -> anyhow::Result<String> {
    let header = Header::new(Algorithm::HS512);
    let result = encode(&header, user, &EncodingKey::from_secret(JWT_SECRET))?;
    Ok(result)
}

