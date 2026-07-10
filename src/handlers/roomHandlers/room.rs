use std::sync::Arc;

use axum::{Extension, Json, Router, response::IntoResponse, routing::{get, post}};
use validator::Validate;

use crate::{dtos::{chatroomDto::{ChatRoomsResponse, CreateRoomRequest, CreateRoomResponse}, userDto::{self, Response}}, errors::{ErrorMessage, HttpError}, models::ChatRoom, state::AppState, utils::middleware::JwtAuthMiddleware};

pub fn room_handler() -> Router {
    Router::new().
    route("/chat", post(create_single_room_route)).
    route("/create-groupchat", post(create_group_chatroom)).
    route("/all-groupchats", get(get_group_rooms))
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
    app_state.db_client.add_roomMembers(result.room_id, Some(user.user.id)).await.map_err(|_| HttpError::server_error(ErrorMessage::AddMemberFailed.return_err()))?;
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

pub async fn create_group_chatroom(Extension(app_state):Extension<Arc<AppState>>, Extension(user): Extension<JwtAuthMiddleware>, Json(body): Json<CreateRoomRequest>) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|_| HttpError::bad_service(ErrorMessage::InvalidCredentials.return_err()))?;
    let body_name = body.room_name;
    let chatroom = app_state.db_client.get_group_chatroom_by_name(body_name.clone()).await.map_err(|_| HttpError::server_error(ErrorMessage::RoomAlreadyExist.return_err()))?;
    if chatroom.is_some(){
        return Err(HttpError::server_error(ErrorMessage::CreateRoomFailed.return_err()));
    }

    let result = app_state.db_client.create_group_chatroom(body_name.clone(), user.user.id, body.description, false, None).await.map_err(|_| HttpError::server_error(ErrorMessage::CreateRoomFailed.return_err()))?;

    app_state.db_client.add_roomMembers(result.room_id, Some(user.user.id)).await.map_err(|_| HttpError::server_error(ErrorMessage::AddMemberFailed.return_err()))?;
    let room = Json(CreateRoomResponse{
        status: "success".to_string(),
        data: result
    });

    Ok(room)   
}

pub async fn get_group_rooms(Extension(appstate): Extension<Arc<AppState>>, Extension(user): Extension<JwtAuthMiddleware>) -> Result<impl IntoResponse, HttpError> {
    let result = appstate.db_client.get_group_chatrooms(Some(user.user.id)).await.map_err(|_| HttpError::server_error(ErrorMessage::RoomNotFound.return_err()))?;
    let response = Json(ChatRoomsResponse{
        status: "success".to_string(),
        data: result
    });

    Ok(response)
}