use std::sync::Arc;

use axum::{Extension, extract::Request, http::{header, request}, middleware::Next, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{errors::{ErrorMessage, HttpError}, models::User, state::AppState, utils::token};

#[derive(Debug, Clone)]
pub struct JwtAuthMiddleware{
    pub user: User
}

// The JWT itself contains stateless data, but your authentication flow is currently stateful because it relies on a mandatory database query (get_user_by_id) on every single request.
// One of the main benefits of a truly stateless JWT setup is that your server can verify a user's identity entirely in memory using just the secret key—without hitting your database at all.
// Later on you can improve on making it properly stateless to avoind overload on the database.

pub async fn auth_middleware(cookie: CookieJar, Extension(app_state): Extension<Arc<AppState>>, mut req: Request, next:Next) -> Result<impl IntoResponse, HttpError>{

    let cookie = cookie.get("token")
    .map(|cookie| cookie.value().to_string())
    .or_else(|| req.headers().get(header::AUTHORIZATION)
    .and_then(|auth_header| auth_header.to_str().ok())
    .and_then(|auth_value| {
         if auth_value.starts_with("Bearer ") {
        Some(auth_value[7..].to_owned())
    }else {
        None
    }
    }
   ));

   let token = cookie.ok_or_else(|| HttpError::unauthorized(ErrorMessage::Unauthorized.return_err()))?;
   let secret = &app_state.envs.jwt_token;
   let token_details = match token::decode_token(&token, secret.as_bytes()) {
       Ok(ok) => ok,
       Err(_) => return Err(HttpError::unauthorized(ErrorMessage::TokenFailed.return_err()))
   };
   let user_id = uuid::Uuid::parse_str(&token_details.sub).map_err(|_| HttpError::unauthorized(ErrorMessage::Unauthorized.return_err()))?;

   let user = app_state.db_client.get_user_by_id(user_id).await.map_err(|_| HttpError::server_error(ErrorMessage::Unauthorized.return_err()))?
   .ok_or_else(|| HttpError::unauthorized(ErrorMessage::UserNotExist.return_err()))?;

   req.extensions_mut().insert(JwtAuthMiddleware{
    user: user.clone()
   });
   

   Ok(next.run(req).await)

}