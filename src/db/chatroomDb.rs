use axum::response::IntoResponse;
use sqlx::query_as;

use crate::{models::ChatRoom, state::DbClient};


impl DbClient{
    pub async fn create_chatroom(&self, room_name: impl Into<Option<String>>, created_by: uuid::Uuid, recipient_id: Option<uuid::Uuid>, description: impl Into<Option<String>>, is_direct: bool) -> Result<ChatRoom, sqlx::Error>{
        let direct_key = if is_direct{
            let recipient = recipient_id.ok_or(sqlx::Error::Protocol(
                "Direct message requires a recipient_id".into()))?;
                let mut key = [created_by.to_string(), recipient.to_string()];
            key.sort();
            let new_key = key.join(":");
            Some(new_key)
        }else{
            None
        };

        let room = query_as!(
            ChatRoom,
            r#"INSERT INTO chatroom (room_name, description, is_direct, direct_key, created_by) 
            VALUES($1, $2, $3, $4, $5)
            ON CONFLICT(direct_key) DO UPDATE SET updated_at = NOW()
            RETURNING room_id, room_name, description, is_direct, direct_key, created_by, created_at, updated_at"#,
            room_name.into(),
            description.into(),
            is_direct,
            direct_key,
            created_by
        ).fetch_one(&self.pool).await?;

        Ok(room)
    }

    pub async fn create_group_chatroom(&self, room_name: impl Into<Option<String>>,  created_by: uuid::Uuid, description: impl Into<Option<String>>, is_direct: bool, direct_key: Option<String>) -> Result<ChatRoom, sqlx::Error>{
        let room = query_as!(
            ChatRoom,
            r#"INSERT INTO chatroom(room_name, description, is_direct, direct_key, created_by) 
            VALUES($1, $2, $3, $4, $5)
            RETURNING room_id, room_name, description, is_direct, direct_key, created_by, created_at, updated_at"#,
            room_name.into(),
            description.into(),
            is_direct,
            direct_key,
            created_by
        ).fetch_one(&self.pool).await?;

        Ok(room)
    }

    pub async fn get_chatroom(&self, created_by: uuid::Uuid, recipient_id: Option<uuid::Uuid>, is_direct: bool) -> Result<Option<ChatRoom>, sqlx::Error> {
        let direct_key = if is_direct{
            let recipient = recipient_id.ok_or(sqlx::Error::Protocol(
                "Direct message requires a recipient_id".into()))?;
            let mut key = [created_by.to_string(), recipient.to_string()];
            key.sort();
            let new_key = key.join(":");
            Some(new_key)
        }else{
            None
        };
        let room = sqlx::query_as!(
            ChatRoom,
            r#"SELECT room_id, room_name, description, is_direct, direct_key, created_by, created_at, updated_at FROM chatroom WHERE direct_key = $1 AND is_direct = true"#,
            direct_key
        ).fetch_optional(&self.pool).await?;
        
        Ok(room)
    }

    pub async fn get_group_chatroom_by_name(&self, room_name: impl Into<Option<String>>) -> Result<Option<ChatRoom>, sqlx::Error>{
        let room = query_as!(
            ChatRoom,
            r#"SELECT room_id, room_name, description, is_direct, direct_key, created_by, created_at, updated_at FROM chatroom WHERE room_name = $1"#,
            room_name.into()
        ).fetch_optional(&self.pool).await?;
        
        Ok(room)
    }

    pub async fn get_group_chatrooms(&self) -> Result<Vec<ChatRoom>, sqlx::Error> {
        let rooms = query_as!(
            ChatRoom,
            r#"SELECT room_id, room_name, description, is_direct, direct_key, created_by, created_at, updated_at FROM chatroom WHERE is_direct = false ORDER BY created_at DESC LIMIT 100"#
        ).fetch_all(&self.pool).await?;

        Ok(rooms)
    }
}
