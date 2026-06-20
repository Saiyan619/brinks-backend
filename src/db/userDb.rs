use chrono::{DateTime, Utc};
use sqlx::query_as;

use crate::{models::User, state::DbClient};


// then create an impl that runs queries functions
impl DbClient {
    pub async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, password_hash, is_verified, verification_token, verification_token_expires, reset_token, reset_token_expires, last_seen, created_at, updated_at FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool) // Added '&' here
        .await?; // Added '?' to handle the DB error

        Ok(user) // This now works because 'user' is an Option<User>
    }

    pub async  fn get_user_by_name(&self, name:String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
           r#"SELECT id, username, email, password_hash, is_verified, verification_token, verification_token_expires, reset_token, reset_token_expires, last_seen, created_at, updated_at FROM users WHERE username = $1"#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;
       Ok(user)
    }

    pub async fn get_user_by_email(&self, email:&str) -> Result<Option<User>, sqlx::Error> {
        let user = query_as!(
            User,
            r#"SELECT id, username, email, password_hash, is_verified, verification_token, verification_token_expires, reset_token, reset_token_expires, last_seen, created_at, updated_at FROM users WHERE email = $1"#,
            email
        ).fetch_optional(&self.pool).await?;

        Ok(user)
    }

    pub async fn get_user_by_token(&self, token:String) -> Result<Option<User>, sqlx::Error> {
        let user = query_as!(
            User,
            r#"SELECT id, username, email, password_hash, is_verified, verification_token, verification_token_expires, reset_token, reset_token_expires, last_seen, created_at, updated_at FROM users  WHERE verification_token = $1"#,
            token
        ).fetch_optional(&self.pool).await?;

        Ok(user)
    }

    pub async  fn get_users(&self) -> Result<Vec<User>, sqlx::Error>{
        let user = query_as!(
            User,
            r#"SELECT id, username, email, password_hash, is_verified, verification_token, verification_token_expires, reset_token, reset_token_expires, last_seen, created_at, updated_at FROM users"#
        ).fetch_all(&self.pool).await?;

        Ok(user)
    }

pub async fn create_user(
    &self, 
    username: impl Into<String>, 
    email: impl Into<String>, 
    password_hash: impl Into<String>, 
    verification_token: impl Into<String>, 
    verification_token_expires: DateTime<Utc>
) -> Result<User, sqlx::Error> {
    let user_result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, verification_token, verification_token_expires)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, username, email, password_hash, is_verified, verification_token, verification_token_expires, reset_token, reset_token_expires, last_seen, created_at, updated_at
        "#,
        username.into(),
        email.into(),
        password_hash.into(),
        verification_token.into(),
        verification_token_expires,
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(user_result)
}

pub async fn verify_user(&self, token:String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET is_verified = true, verification_token = null, verification_token_expires = null WHERE verification_token = $1"#,
        token
    )
    .execute(&self.pool)
    .await?;
    Ok(())
}
}