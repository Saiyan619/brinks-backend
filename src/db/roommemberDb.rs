use sqlx::{query, query_as};

use crate::{models::{RoomMembers, User}, state::DbClient};


impl DbClient{
    pub async fn add_roomMembers(&self, room_id: uuid::Uuid, user_id: Option<uuid::Uuid>) -> Result<RoomMembers, sqlx::Error>{
        let roomMembers = query_as!(
            RoomMembers,
            r#"INSERT INTO room_members(room_id, user_id) VALUES($1, $2) RETURNING room_id, user_id, joined_at"#,
            room_id,
            user_id
        ).fetch_one(&self.pool).await?;

        Ok(roomMembers)
    }

    pub async fn is_already_member(&self, user_id: Option<uuid::Uuid>, room_id: uuid::Uuid) -> Result<bool, sqlx::Error>{
        let member = query!(
            r#"SELECT EXISTS( SELECT 1 FROM room_members WHERE user_id = $1 AND room_id = $2)"#,
            user_id,
            room_id
        ).fetch_one(&self.pool).await?;

        match member.exists {
            Some(exists) => Ok(exists),
            None => Ok(false),
}
    }
}