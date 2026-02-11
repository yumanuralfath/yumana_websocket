use crate::room::RoomManager;
use crate::types::{ClientMessage, ServerMessage};
use crate::Player;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::Message;

pub struct MessageHandler {
    pub user_id: String,
    pub username: String,
    pub room_manager: Arc<RoomManager>,
}

impl MessageHandler {
    pub async fn handle(&self, msg: ClientMessage) {
        match msg {
            ClientMessage::JoinRoom { room_id } => {
                let player = Player {
                    id: self.user_id.clone(),
                    username: "User ".to_string() + &self.user_id,
                };

                if (self.room_manager.join_room(&room_id, player).await).is_ok() {
                    let response = ServerMessage::RoomJoined {
                        room_id: room_id.clone(),
                        players: vec![], // Bisa diisi list player asli
                    };
                    let json = serde_json::to_string(&response).unwrap();
                    self.room_manager
                        .conn_manager
                        .send_to(&self.user_id, Message::Text(json));
                }
            }
            ClientMessage::SendMessage { content } => {
                // Sekarang ini akan berhasil karena client_rooms sudah terisi
                if let Some(room_id) = self.room_manager.client_rooms.get(&self.user_id) {
                    let chat = ServerMessage::ChatMessage {
                        from: self.username.clone(),
                        content,
                        timestamp: chrono::Utc::now().timestamp(),
                    };
                    let json = serde_json::to_string(&chat).unwrap();
                    self.room_manager
                        .broadcast(&room_id, Message::Text(json))
                        .await;
                }
            }
            _ => {}
        }
    }
}
