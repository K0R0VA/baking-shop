use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, Deserialize};
use crate::models::CurrentUser;

const JWT_SECRET: &[u8; 6] = b"secret";

#[derive(Serialize)]
pub struct SerializedClaims<'a> {
    exp: usize,
    user: &'a CurrentUser,
}

#[derive(Deserialize)]
pub struct DeserializedClaims {
    exp: usize,
    user: CurrentUser,
}

pub fn create_jwt(user: &CurrentUser) -> anyhow::Result<String> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60 * 60 * 12))
        .expect("valid timestamp")
        .timestamp() as usize;
    let header = Header::new(Algorithm::HS512);
    let result = encode(&header, &SerializedClaims {user, exp}, &EncodingKey::from_secret(JWT_SECRET))?;
    Ok(result)
}

pub fn decode_identity(identity: String) -> jsonwebtoken::errors::Result<CurrentUser>{
    decode::<DeserializedClaims>(&identity,
                     &DecodingKey::from_secret(JWT_SECRET),
                     &Validation::new(Algorithm::HS512))
        .map(|token| token.claims.user)
}

