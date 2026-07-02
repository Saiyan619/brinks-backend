use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::ChatRoom;


#[derive(Debug, Validate, Deserialize)]
pub struct CreateRoomRequest{
    #[validate(length(min = 3, max = 50, message = "Name must be 3-50 characters"))]
    pub room_name: Option<String>,
    
    #[validate(length(max = 300))]
    pub description: Option<String>,
    
    pub is_direct: bool,

    pub created_by: uuid::Uuid,

    pub recipient: Option<uuid::Uuid>
}

#[derive(Serialize)]
pub struct CreateRoomResponse {
    pub status: String,
    pub data: ChatRoom, 
}

#[derive(Serialize)]
pub struct ChatRoomResponse{
    pub status: String,
    pub data: ChatRoom
}
#[derive(Serialize)]
pub struct ChatRoomsResponse{
    pub status: String,
    pub data: Vec<ChatRoom>
}