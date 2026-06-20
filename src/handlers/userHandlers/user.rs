use std::sync::Arc;

use axum::{Extension, Json, Router, response::IntoResponse, routing::{post, get}};

use crate::{dtos::userDto::{UserDto, UserResponseDto, UsersResponseDto}, errors::{ErrorMessage, HttpError}, state::AppState, utils::middleware::JwtAuthMiddleware};

pub fn user_handlers() -> Router {
    Router::new()
    .route("/me", get(get_me))
    .route("/user-all", get(get_users))
}

async fn get_me(Extension(_app_state):Extension<Arc<AppState>>, Extension(user): Extension<JwtAuthMiddleware>) -> Result<impl IntoResponse, HttpError> {
    
    let filtered_user = UserDto::filter_user(&user.user);
    let response = UserResponseDto{
        status: "success".to_string(),
        data: filtered_user
    };
    Ok(Json(response))
}



async fn get_users(Extension(app_state):Extension<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
    let result = app_state.db_client.get_users().await.map_err(|_| HttpError::server_error(ErrorMessage::FetchAllUsersFailed.return_err()))?;
    let filtered_users = UserDto::filter_users(&result);
    let users_len = filtered_users.len() as i64;
    let response = UsersResponseDto{
        status: "success".to_string(),
        data: filtered_users,
        result: users_len
    };
    Ok(Json(response))
    
}