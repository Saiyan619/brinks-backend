use std::sync::Arc;

use axum::{Extension, Json, Router, extract::Query, http::{HeaderMap, header::SET_COOKIE, response}, response::IntoResponse, routing::{Route, post, get}};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{Duration, Utc};
use uuid::Timestamp;
use validator::Validate;

use crate::{dtos::userDto::{LoginReponseDto, LoginUserDto, RegisterUserDto, Response, VerifyEmailQueryDto}, errors::{ErrorMessage, HttpError}, mail::sendEmail::send_email_verification, state::AppState, utils::{password::{compare_password, hash_password}, token::create_token}};

pub fn auth_handlers() -> Router {
    Router::new()
    .route("/register", post(register))
    .route("/login", post(login))
    .route("/verify-email", get(verify_email))
}
pub async fn register(Extension(app_state): Extension<Arc<AppState>>, Json(body):Json<RegisterUserDto>) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_service(e.to_string()))?;

    let new_user_hashed_password = hash_password(&body.password).map_err(|e| HttpError::bad_service(e.return_err()))?;

    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);
    let result = app_state.db_client.create_user(body.username, &body.email, new_user_hashed_password, verification_token.clone(), expires_at).await;
    match result {
        Ok(_) => {
            // send_email_verification(vec![&body.email], "onboarding@resend.dev".to_string(), verification_token, "verify-email".to_string()).await.map_err(|_| HttpError::bad_service(ErrorMessage::EmailError.return_err()))?;
            send_email_verification(body.email, verification_token).await.map_err(|_| HttpError::bad_service(ErrorMessage::EmailError.return_err()))?;
            Ok(Json(Response{
                status: "success".to_string(),
                message: "User is registered successfully!, Please check your email to verify your account".to_string()
            }))
        },
        Err(sqlx::Error::Database(db_err)) => {
          if db_err.is_unique_violation(){
            Err(HttpError::server_error(ErrorMessage::UserAlreadyExist.return_err()))
          }else {
              Err(HttpError::server_error(db_err.to_string()))
          }
        }
        Err(e) => Err(HttpError::server_error(e.to_string()))
    }
}

pub async fn login(Extension(app_state): Extension<Arc<AppState>>, Json(body): Json<LoginUserDto>) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|_| HttpError::bad_service(ErrorMessage::InvalidCredentials.return_err()))?;
    let user = app_state.db_client.get_user_by_email(&body.email).await.map_err(|e| HttpError::server_error(e.to_string()))?.ok_or_else(|| HttpError::unauthorized(ErrorMessage::InvalidCredentials.return_err()))?;
    let is_password_valid = compare_password(&body.password, &user.password_hash)
    .map_err(|_| HttpError::unauthorized(ErrorMessage::InvalidCredentials.return_err()))?;
    if !is_password_valid {
    return Err(HttpError::unauthorized(ErrorMessage::InvalidCredentials.return_err()));
    }
    let secret = &app_state.envs.jwt_token;
    let maxage = app_state.envs.jwt_maxage;
    let token = create_token(user.id.to_string(), secret.as_bytes(),  maxage).map_err(|_| HttpError::server_error(ErrorMessage::TokenFailed.return_err()))?;
    let cookie_duration = time::Duration::minutes(maxage * 60);

    let cookie = Cookie::build(("token", &token)).path("/").max_age(cookie_duration).http_only(true).same_site(SameSite::Lax).build();//When going to production remove same-site, only added it because i was 
    //issues with saving the token in the cookie storage in my browser while building the frontend 
    //also remember to add "secure" too

    let response = axum::response::Json(LoginReponseDto{
        status: "success".to_string(),
        token: token.clone()
    });

    let mut header = HeaderMap::new();

    header.append(SET_COOKIE, cookie.to_string().parse().unwrap());

    let mut response = response.into_response();
    response.headers_mut().extend(header); 

    Ok(response)

}

pub async fn verify_email(Extension(app_state): Extension<Arc<AppState>>, Query(body): Query<VerifyEmailQueryDto>) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_service(e.to_string()))?;
    let result = app_state.db_client.get_user_by_token(body.token.clone()).await.map_err(|e| HttpError::server_error(e.to_string()))?;
    let user = result.ok_or(HttpError::server_error(ErrorMessage::UserNotExist.return_err()))?;

    let secret = &app_state.envs.jwt_token;
    let maxage = app_state.envs.jwt_maxage;
    let cookie_duration = time::Duration::hours(maxage * 60);
    let inject_token = create_token(user.id.to_string(), secret.as_bytes(), maxage).map_err(|_| HttpError::server_error(ErrorMessage::TokenFailed.return_err()))?;
    let cookie = Cookie::build(("token", inject_token.to_string())).path("/").max_age(cookie_duration).http_only(true).build();

    app_state.db_client.verify_user(body.token)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut header = HeaderMap::new();

    header.append(SET_COOKIE, cookie.to_string().parse().unwrap());

    let response = Json(VerifyEmailQueryDto{
        token: inject_token
    });

    let mut response = response.into_response();

    response.headers_mut().extend(header);

    Ok(response)
}


// ?              →  unwrap Ok or return Err immediately
// ok_or          →  Option to Result with simple error
// ok_or_else     →  Option to Result with complex error
// map_err        →  convert one error type to another
// map_or_else    →  extract final value from Option or Result
// unwrap_or_else →  get value or return a default
// and_then       →  chain operations that can each fail
// map            →  transform the success value