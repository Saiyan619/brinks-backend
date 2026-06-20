use std::{mem, sync::Arc};

use axum::{Extension, Json, Router, extract::Path, response::IntoResponse, routing::get};
use uuid::Uuid;
use validator::Validate;

use crate::{dtos::messageDto::{MessageRequestDto, MessageResponseDto, MessagesResponseDto}, errors::{ErrorMessage, HttpError}, state::AppState, utils::middleware::JwtAuthMiddleware};

pub fn message_handler() -> Router {
    Router::new().
    route("/messages/:room_id", get(get_message))
}

// pub async fn create_message(Extension(app_state): Extension<Arc<AppState>>, Extension(user): Extension<JwtAuthMiddleware>, Json(body): Json<MessageRequestDto>) -> Result<impl IntoResponse, HttpError>{
//     body.validate().map_err(|e| HttpError::bad_service(e.to_string()))?;
//     let user = app_state.db_client.get_user_by_id(user.user.id).await.map_err(|e| HttpError::server_error(e.to_string()))?.ok_or_else(|| HttpError::unauthorized(ErrorMessage::InvalidCredentials.return_err()))?;

//     // let created_room = app_state.db_client.create_chatroom(None, user.id, None, true).await.map_err(|e| HttpError::server_error(e.to_string()))?;

//     let created_message = app_state.db_client.create_message(body.message, user.id).await.map_err(|_| HttpError::server_error(ErrorMessage::FailedMessage.return_err()))?;
    
//     match  {
        
//     }
// }

pub async fn get_message(Extension(app_state): Extension<Arc<AppState>>, Extension(user): Extension<JwtAuthMiddleware>, Path(room_id): Path<Uuid>) -> Result<impl IntoResponse, HttpError> {
    let msg = app_state.db_client.get_message(room_id).await.map_err(|_| HttpError::server_error(ErrorMessage::FailedToGetMsg.return_err()))?;
    let filtered_msgs = MessageResponseDto::filter_msgs(&msg);
    let response: Json<MessagesResponseDto> = Json(
        MessagesResponseDto { 
            status: "success".to_string(), 
            data: filtered_msgs
        }
    );
    Ok(response)
}