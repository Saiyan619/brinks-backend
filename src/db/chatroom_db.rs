use axum::response::IntoResponse;
use sqlx::query_as;

use crate::{models::{ChatRoom, ChatRoomWithOtherUser, GroupChatroomWithMembership}, state::DbClient};


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

    pub async fn get_user_direct_chatrooms(&self, user_id: Option<uuid::Uuid>) -> Result<Vec<ChatRoomWithOtherUser>, sqlx::Error> {
    let rooms = query_as!(
        ChatRoomWithOtherUser,
        r#"
        SELECT
            chatroom.room_id,
            chatroom.room_name,
            chatroom.description,
            chatroom.is_direct,
            chatroom.direct_key,
            chatroom.created_by,
            chatroom.created_at,
            chatroom.updated_at,
            other_user.username AS "other_username?"
        FROM chatroom
        JOIN room_members my_membership
            ON my_membership.room_id = chatroom.room_id
        LEFT JOIN room_members other_membership
            ON other_membership.room_id = chatroom.room_id
            AND other_membership.user_id != my_membership.user_id
        LEFT JOIN users other_user
            ON other_user.id = other_membership.user_id
        WHERE my_membership.user_id = $1 AND chatroom.is_direct = true
        ORDER BY chatroom.created_at DESC
        "#,
        user_id
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(rooms)
}

pub async fn get_user_group_chatrooms(&self, user_id: Option<uuid::Uuid>) -> Result<Vec<ChatRoomWithOtherUser>, sqlx::Error> {
    let rooms = query_as!(
        ChatRoomWithOtherUser,
        r#"
        SELECT
            chatroom.room_id,
            chatroom.room_name,
            chatroom.description,
            chatroom.is_direct,
            chatroom.direct_key,
            chatroom.created_by,
            chatroom.created_at,
            chatroom.updated_at,
            other_user.username AS "other_username?"
        FROM chatroom
        JOIN room_members my_membership
            ON my_membership.room_id = chatroom.room_id
        LEFT JOIN room_members other_membership
            ON other_membership.room_id = chatroom.room_id
            AND other_membership.user_id != my_membership.user_id
        LEFT JOIN users other_user
            ON other_user.id = other_membership.user_id
        WHERE my_membership.user_id = $1 AND chatroom.is_direct = false
        ORDER BY chatroom.created_at DESC
        "#,
        user_id
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(rooms)
}

    // FIX for checking and assigning if user is a member to any room(this was the best solution i can come up with rn)
    // Quick recap of what actually fixed it, in case it's useful for next time: 
    // is_member gets computed live from room_members via a correlated EXISTS subquery, 
    // kept off the shared ChatRoom struct (so it doesn't break your other queries), 
    // and lives on its own GroupChatroomWithMembership struct + response type instead — 
    // then the frontend just reads that boolean straight off each group to decide "Open" vs "Join Group."
    pub async fn get_group_chatrooms(&self, user_id: Option<uuid::Uuid>) -> Result<Vec<GroupChatroomWithMembership>, sqlx::Error> {
        let rooms = query_as!(
            GroupChatroomWithMembership,
            r#"SELECT  chatroom.room_id, chatroom.room_name, chatroom.description, chatroom.is_direct,
            chatroom.direct_key, chatroom.created_by, chatroom.created_at, chatroom.updated_at, 
            EXISTS (
            SELECT 1 FROM room_members
            WHERE room_members.room_id = chatroom.room_id AND room_members.user_id = $1
            ) AS "is_member!" FROM chatroom
            WHERE is_direct = false 
            ORDER BY created_at DESC LIMIT 100"#,
            user_id
        ).fetch_all(&self.pool).await?;

        Ok(rooms)
    }
}
