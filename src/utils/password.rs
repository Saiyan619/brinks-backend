use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};

use crate::errors::{ErrorMessage, HttpError};

pub fn hash_password(password:&str) -> Result<String, ErrorMessage>{
    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }
    let salt = SaltString::generate(&mut OsRng);

    let argon = Argon2::default();

    let pasword_hash = argon.hash_password(&password.as_bytes(), &salt).map_err(|_| ErrorMessage::HashingError)?.to_string();

    Ok(pasword_hash)
}

pub fn compare_password(password: &str, stored_password:&str) -> Result<bool, ErrorMessage> {
    if password.is_empty(){
        return Err(ErrorMessage::EmptyPassword);
    }
    let passworrd_hash = PasswordHash::new(&stored_password).map_err(|_| ErrorMessage::HashingError)?;
    let compared = Argon2::default().verify_password(password.as_bytes(), &passworrd_hash).is_ok();
    Ok(compared)
}