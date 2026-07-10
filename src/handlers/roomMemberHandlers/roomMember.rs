use std::sync::Arc;

use axum::{Extension, Json, Router, response::IntoResponse, routing::post};

use crate::{dtos::roomMemberDto::{RoomMembersRequest, roomMemberResponse}, errors::{ErrorMessage, HttpError}, state::AppState, utils::middleware::JwtAuthMiddleware};

pub fn room_members_handler() -> Router {
    Router::new().
    route("/join", post(join_room))
}

pub async fn join_room(Extension(appstate): Extension<Arc<AppState>>, Extension(user): Extension<JwtAuthMiddleware>, Json(body): Json<RoomMembersRequest>) -> Result<impl IntoResponse, HttpError>{
    let already_member = appstate.db_client.is_already_member(Some(body.user_id), body.room_id).await.map_err(|_| HttpError::server_error(ErrorMessage::MemberNotExist.return_err()))?;
    if already_member {
        return Err(HttpError::bad_service("user already a member".to_string()));
    }
    let result = appstate.db_client.add_roomMembers(body.room_id, Some(user.user.id)).await.map_err(|_| HttpError::server_error(ErrorMessage::AddMemberFailed.return_err()))?;
    let result_json = Json(roomMemberResponse{
        status: "success".to_string(),
        data: result
    });

    Ok(result_json)
}