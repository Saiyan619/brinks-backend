use sqlx::query_as;

use crate::{models::RoomMembers, state::DbClient};


impl DbClient{
    pub async fn add_roomMembers(&self, room_id: uuid::Uuid, user_id: uuid::Uuid) -> Result<RoomMembers, sqlx::Error>{
        let roomMembers = query_as!(
            RoomMembers,
            r#"INSERT INTO room_members(room_id, user_id) VALUES($1, $2) RETURNING room_id, user_id, joined_at"#,
            room_id,
            user_id
        ).fetch_one(&self.pool).await?;

        Ok(roomMembers)
    }
}