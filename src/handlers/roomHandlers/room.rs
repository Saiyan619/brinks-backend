use std::sync::Arc;

use axum::{Extension, Json, Router, response::IntoResponse, routing::post};
use validator::Validate;

use crate::{dtos::{chatroomDto::{CreateRoomRequest, CreateRoomResponse}, userDto::{self, Response}}, errors::{ErrorMessage, HttpError}, models::ChatRoom, state::AppState, utils::middleware::JwtAuthMiddleware};

pub fn room_handler() -> Router {
    Router::new().
    route("/chat", post(create_single_room_route))
    
}

pub async fn create_single_room_route(Extension(app_state):Extension<Arc<AppState>>, Extension(user): Extension<JwtAuthMiddleware>, Json(body): Json<CreateRoomRequest>) -> Result<impl  IntoResponse, HttpError> {
    body.validate().map_err(|_| HttpError::bad_service(ErrorMessage::InvalidInput.return_err()))?;
    // let filtered_user = userDto::UserDto::filter_user(&user.user);
    let room_result = app_state.db_client.get_chatroom(user.user.id, body.recipient, true).await.map_err(|_| HttpError::server_error(ErrorMessage::RoomNotFound.return_err()))?;
    if let Some(room) = room_result {
    // Room exists — just return it
    return Ok(Json(CreateRoomResponse {
        status: "success".to_string(),
        data: room,
    }));
}
    let result = app_state.db_client.create_chatroom(None, user.user.id, body.recipient, None, true).await.map_err(|_| HttpError::server_error(ErrorMessage::CreateRoomFailed.return_err()))?;
    app_state.db_client.add_roomMembers(result.room_id, user.user.id).await.map_err(|_| HttpError::server_error(ErrorMessage::AddMemberFailed.return_err()))?;
    let user_result = app_state.db_client.get_user_by_id(body.recipient).await.map_err(|_| HttpError::server_error(ErrorMessage::UserNotExist.return_err()))?;
    let get_user = user_result.ok_or_else(|| HttpError::server_error(ErrorMessage::UserNotExist.return_err()))?;
    if get_user.is_verified{
        app_state.db_client.add_roomMembers(result.room_id, body.recipient).await.map_err(|_| HttpError::server_error(ErrorMessage::AddMemberFailedV2.return_err()))?;
        println!("recipient has also been added to the roommemberDb!!")
    }
    let chat: Json<CreateRoomResponse> = Json(CreateRoomResponse{
        status: "success".to_string(),
        data: result
    });

    Ok(chat)
}