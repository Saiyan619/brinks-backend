use sqlx::query_as;

use crate::{models::Message, state::DbClient};


impl DbClient {
    pub async fn create_message(&self, room_id: uuid::Uuid, message: impl Into<String>, sender_id: uuid::Uuid) -> Result<Message, sqlx::Error>{
        let messages = query_as!(
            Message,
            r#"INSERT INTO messages (message, room_id, sender_id) VALUES ($1, $2, $3) RETURNING messages_id, room_id, message, sender_id, updated_at, created_at"#,
            message.into(),
            room_id,
            sender_id
        ).fetch_one(&self.pool).await?;

        Ok(messages)
    }

    pub async fn get_message(&self, room_id: uuid::Uuid) -> Result<Vec<Message>, sqlx::Error> {
        let message = query_as!(
            Message,
            r#"SELECT messages_id, room_id, message, sender_id, updated_at, created_at FROM messages WHERE room_id = $1"#,
            room_id
        ).fetch_all(&self.pool).await?;

        Ok(message)
    }
}

// pub struct message{
//     pub messages_id: uuid::Uuid,
//     pub room_id: uuid::Uuid,
//     pub message: String,
//     pub sender_id: uuid::Uuid,
//     pub updated_at: DateTime<Utc>,
//     pub created_at: DateTime<Utc>
// }