use std::sync::Arc;

use axum::{Extension, Json, Router, middleware, response::IntoResponse, routing::{get}};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

use crate::{errors::HttpError, handlers::{message_handlers::messages::message_handler, room_handlers::room::room_handler, room_member_handlers::room_member::room_members_handler, user_handlers::{auth::auth_handlers, user::user_handlers}, web_socket::webSocket::ws_handler}, state::AppState, utils::middleware::{auth_middleware}};

pub fn health_handler() -> Router {
    Router::new().route("/", get(api_health))
}
#[derive(Debug, Serialize, Deserialize)]
struct HealthMessageResponse{
    message: String
}
pub async fn api_health() -> Result<impl IntoResponse, HttpError> {
    let health_reponse = "This api is healthy and responsive";
    Ok(Json(HealthMessageResponse{
        message: health_reponse.to_string()
    }))
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let routes = Router::new()
    .nest("/auth", auth_handlers())
    .nest("/users", user_handlers().layer(middleware::from_fn(auth_middleware)))
    .nest("/chatroom", room_handler().layer(middleware::from_fn(auth_middleware)))
    .nest("/ws", ws_handler().layer(middleware::from_fn(auth_middleware)))
    .nest("/message", message_handler().layer(middleware::from_fn(auth_middleware)))
    .nest("/add-member", room_members_handler().layer(middleware::from_fn(auth_middleware)))
    .nest("/health", health_handler())
    .layer(TraceLayer::new_for_http())
    .layer(Extension(app_state));

    Router::new().nest("/api", routes)
}
