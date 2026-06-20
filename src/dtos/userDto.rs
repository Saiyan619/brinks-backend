use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{dtos::userDto, models::User};

// #[validate(...)]: These are attributes provided by the validator crate.
//  They define the "rules" for your data. 
//  Common ones include:
//  email: Checks for @ and valid domains.
//  length(min = x, max = y): Sets character limits.
// range(min = x, max = y): For numbers.url: Checks for valid URL format.
#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct RegisterUserDto{
    #[validate(length(min = 2, message = "username is required"))]
    pub username: String,
    #[validate(length(min = 10, message = "email is required"), email(message = "invalid format!"))]
    pub email: String,
    #[validate(length(min = 1, message = "password is required"), length(min = 6, message = "password must be more than 6 characters"))]
    pub password: String
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct LoginUserDto{
    #[validate(length(min = 10, message = "email is required"), email(message = "invalid format!"))]
    pub email: String,
    #[validate(length(min = 1, message = "password is required"), length(min = 6, message = "password must be more than 6 characters"))]
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_verified: bool,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl UserDto {
    pub fn filter_user(user:&User) -> Self{
        UserDto {id: user.id.to_string(), username:user.username.clone(), email:user.email.clone(), is_verified:user.is_verified, last_seen:user.last_seen, created_at:user.created_at, updated_at:user.updated_at }
    }
    
    pub fn filter_users(users:&[User]) -> Vec<Self> {
        users.iter().map(UserDto::filter_user).collect()
    }
}



#[derive(Debug, Validate)]
pub struct InfiniteScrollDto {
    #[validate(range(min = 1, max = 50))]
    pub limit: usize,

    pub after_id: Option<uuid::Uuid>, // "Give me items AFTER this specific ID, if theres none before start from the beginning..ig"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response{
    pub status: String,
    pub message: String
}

#[derive(Debug, Serialize, Clone)]
pub struct LoginReponseDto{
    pub status: String,
    pub token: String
}

#[derive(Debug, Serialize)]
pub struct UserResponseDto{
    pub status: String,
    pub data: UserDto
}


#[derive(Debug, Serialize)]
pub struct UsersResponseDto{
    pub status: String,
    pub data: Vec<UserDto>,
    pub result: i64
}


#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required."),)]
    pub token: String,
}

// struct userDto{
//     pub id: String,
//     pub username: String,
//     pub email: String,
//     pub password_hash: String,
//     pub is_verified: Option<DateTime<Utc>>,
//     pub verification_token: String,
//     pub verification_token_expires: DateTime<Utc>,
//     pub reset_token: Option<DateTime<Utc>>,
//     pub reset_token_expires: DateTime<Utc>,
//     pub last_seen: DateTime<Utc>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>
// }