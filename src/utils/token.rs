use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorMessage, HttpError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub sub: String,
    pub iat: i64,
    pub exp: i64
}

 pub fn create_token(user_id:String, secret: &[u8], expires_in_minutes:i64) -> Result<String, jsonwebtoken::errors::Error>{
    if user_id.is_empty(){
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::minutes(expires_in_minutes)).timestamp();
    let claim = Claims{
        sub: user_id,
        iat,
        exp
    };

    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(secret))?;
    Ok(token)
}

pub fn decode_token(token: &str, secret: &[u8]) -> Result<Claims, HttpError> {
 let decode = decode::<Claims>(token, &DecodingKey::from_secret(secret), &Validation::new(Algorithm::HS256));
 match decode {
     Ok(ok) => Ok(ok.claims),
     Err(_) => return Err(HttpError::unauthorized(ErrorMessage::Unauthorized.return_err()))
 }
}