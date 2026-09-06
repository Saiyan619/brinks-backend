use chrono::{DateTime, Utc};
use serde::Serialize;
use validator::Validate;

use crate::models::Message;

#[derive(Debug, Validate)]
pub struct MessageRequestDto{
    #[validate(length(min = 1, message = "Can't send an empty text"))]
    pub message: String,
    pub sender_id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub recipient_id: uuid::Uuid
}

#[derive(Debug, Serialize)]
pub struct MessageResponseDto{
    pub messages_id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub message: String,
    pub sender_id: Option<uuid::Uuid>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

impl MessageResponseDto {
    pub fn filter_msg(message:&Message) -> Self{
        MessageResponseDto {messages_id: message.messages_id.clone(), room_id:message.room_id.clone(), message:message.message.clone(), sender_id: message.sender_id.clone(), created_at:message.created_at, updated_at:message.updated_at }
    }
    
    pub fn filter_msgs(messages:&[Message]) -> Vec<Self> {
        messages.iter().map(MessageResponseDto::filter_msg).collect()
    }
}

#[derive(Debug, Serialize)]
pub struct MessagesResponseDto{
    pub status: String,
    pub data: Vec<MessageResponseDto>,
}

 
// pub struct message{
//     pub messages_id: uuid::Uuid,
//     pub room_id: uuid::Uuid,
//     pub message: String,
//     pub sender_id: uuid::Uuid,
//     pub updated_at: DateTime<Utc>,
//     pub created_at: DateTime<Utc>
// }