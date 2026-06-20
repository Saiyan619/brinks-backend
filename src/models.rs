use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User{
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub verification_token_expires: Option<DateTime<Utc>>,
    pub reset_token: Option<String>,
    pub reset_token_expires: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ChatRoom{
    pub room_id: uuid::Uuid,
    pub room_name: Option<String>,
    pub description: Option<String>,
    pub is_direct: bool,
    pub direct_key: Option<String>,
    pub created_by: Option<uuid::Uuid>,
    //Comeback to this: fix the migration to a not null - its a time variable which postgress would create on auto so option isnt necessary
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>
}


#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Message{
    pub messages_id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub message: String,
    pub sender_id: Option<uuid::Uuid>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

// messages_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
//     room_id UUID NOT NULL REFERENCES chatroom(room_id) ON DELETE CASCADE,
//     message TEXT NOT NULL,
//     sender_id UUID REFERENCES users(id) ON DELETE SET NULL,
//     updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
//     created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RoomMembers{
    pub room_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub joined_at: DateTime<Utc>,
}

// room_id UUID NOT NULL REFERENCES chatroom(room_id) ON DELETE CASCADE,
//     user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
//     joined_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
//     PRIMARY KEY (room_id, user_id)