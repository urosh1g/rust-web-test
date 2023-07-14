use crate::models::claim::UserClaims;
use actix_web::cookie::Cookie;

use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};

pub fn verify_token(cookie: Cookie) -> Result<TokenData<UserClaims>, Error> {
    let data = validate_token(cookie.value())?;
    Ok(data)
}

pub fn validate_token(token: &str) -> Result<TokenData<UserClaims>, Error> {
    let validator = Validation::new(Algorithm::HS256);
    let token_data = decode::<UserClaims>(
        token,
        &DecodingKey::from_secret("secret".as_bytes()),
        &validator,
    )?;
    Ok(token_data)
}

pub fn generate_token(claims: UserClaims) -> Result<String, Error> {
    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret("secret".as_bytes());
    encode(&header, &claims, &key)
}
