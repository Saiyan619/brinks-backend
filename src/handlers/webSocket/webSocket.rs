use std::sync::Arc;

use axum::{Extension, Json, Router, body, extract::{Path, WebSocketUpgrade, ws::{Message, WebSocket}}, response::IntoResponse, routing::get};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use validator::Validate;

use crate::{dtos::messageDto::MessageRequestDto, errors::{ErrorMessage, HttpError}, state::AppState, utils::middleware::JwtAuthMiddleware};

pub fn ws_handler() -> Router {
    Router::new().
    route("/wschat/:room_id", get(web_handler))
}

pub async fn web_handler(ws: WebSocketUpgrade, 
    Path(room_id): Path<uuid::Uuid>, 
    Extension(app_state): Extension<Arc<AppState>>, 
    Extension(user): Extension<JwtAuthMiddleware>,
    // Json(body):Json<MessageRequestDto>
) -> impl IntoResponse {
    // body.validate().map_err(|_| HttpError::bad_service(ErrorMessage::SocketFailed.return_err()));
        let sender_id = user.user.id;
    ws.on_upgrade(move |socket| handle_socket(socket,  app_state, room_id, sender_id))
}

pub async fn handle_socket(socket: WebSocket, app_state: Arc<AppState>, room_id: uuid::Uuid, sender_id: uuid::Uuid) {
    //Splits sockets
    let (mut sender, mut reciever) = socket.split();
    // Disallows more than 1 writes at a time
    let mut room_guard = app_state.tx.lock().await;
    //Finds or create room then inserts into broadcast
    let tx = room_guard.entry(room_id.to_string()).or_insert_with(|| {let (tx, _) = broadcast::channel(100); tx}).clone();
    drop(room_guard);
    
    //Subscribes to the broadcast
    let mut rx = tx.subscribe();

    // rx.recv() → picks up from broadcast → sender.send() → User A's screen
    tokio::spawn(async move{
        while let Ok(msg) = rx.recv().await {
        if sender.send(Message::Text(msg)).await.is_err() {
            println!("something weent wrong with the spawn loop");
            break;
        }
    }
    });
    
    // receiver.next() → User A typed something → tx.send() → broadcast channel
    while let Some(Ok(Message::Text(text))) = reciever.next().await {
        
        let result = app_state.db_client.create_message(room_id, text.to_string(), sender_id).await;
        match result {
            Ok(msg) => {
               let payload = serde_json::to_string(&msg).unwrap();
                if tx.send(payload).is_err(){
                    break;
                }
            },
            Err(err) =>{
                eprintln!("db error: {:?}", err);
            }
        };
        }

        
}


// Verify ws - thankfully i already wrote a jwt middleware that helps with this





// Lock the HashMap so you can safely look inside it.
// Check if the room_id exists.If it DOES NOT exist: Create a brand new broadcast::channel, wrap it in your ChatRoom struct, and .insert() it into the HashMap.
// If it DOES exist: Grab the existing tx sender for that room.
// Subscribe to that room's channel (let mut rx = tx.subscribe()).
// Drop the lock immediately so other users can connect to other rooms one-by-one without blocking the whole server.
// Spawn your background tasks using that specific room's tx and rx.