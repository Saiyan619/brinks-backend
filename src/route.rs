use std::sync::Arc;

use axum::{Extension, Router, middleware};
use tower_http::trace::TraceLayer;

use crate::{handlers::{messageHandlers::messages::message_handler, roomHandlers::room::room_handler, roomMemberHandlers::roomMember::room_members_handler, userHandlers::{auth::auth_handlers, user::user_handlers}, webSocket::webSocket::ws_handler}, state::AppState, utils::middleware::{JwtAuthMiddleware, auth_middleware}};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    let routes = Router::new()
    .nest("/auth", auth_handlers())
    .nest("/users", user_handlers().layer(middleware::from_fn(auth_middleware)))
    .nest("/chatroom", room_handler().layer(middleware::from_fn(auth_middleware)))
    .nest("/ws", ws_handler().layer(middleware::from_fn(auth_middleware)))
    .nest("/message", message_handler().layer(middleware::from_fn(auth_middleware)))
    .nest("/add-member", room_members_handler().layer(middleware::from_fn(auth_middleware)))
    .layer(TraceLayer::new_for_http())
    .layer(Extension(app_state));

    Router::new().nest("/api", routes)
}
