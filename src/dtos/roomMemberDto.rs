use serde::{Deserialize, Serialize};

use crate::models::RoomMembers;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomMembersRequest{
    pub room_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
}

#[derive(Debug, Serialize)]
pub struct roomMemberResponse{
    pub status: String,
    pub data: RoomMembers
}