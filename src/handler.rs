use crate::room::RoomManager;
use crate::types::{ClientMessage, RoomType, ServerMessage};
use std::sync::Arc;

pub struct MessageHandler {
    client_id: String,
    room_manager: Arc<RoomManager>,
}

impl MessageHandler {
    pub fn new(client_id: String, room_manager: Arc<RoomManager>) -> Self {
        Self {
            client_id,
            room_manager,
        }
    }

    pub async fn handle_message(&self, message: ClientMessage) -> Option<Vec<ServerMessage>> {
        match message {
            ClientMessage::CreateRoom {
                room_type,
                max_players,
            } => self.handle_create_room(room_type, max_players).await,
            ClientMessage::JoinRoom { room_id } => self.handle_join_room(&room_id).await,
            ClientMessage::LeaveRoom => self.handle_leave_room().await,
            ClientMessage::SendMessage { content } => self.handle_send_message(&content).await,
            ClientMessage::GameAction { action, data } => {
                self.handle_game_action(&action, data).await
            }
        }
    }

    async fn handle_create_room(
        &self,
        room_type: RoomType,
        max_players: Option<usize>,
    ) -> Option<Vec<ServerMessage>> {
        let room_id = self
            .room_manager
            .create_room(room_type.clone(), max_players);

        // Auto-join the creator
        if let Err(e) = self.room_manager.join_room(&room_id, &self.client_id).await {
            return Some(vec![ServerMessage::Error { message: e }]);
        }

        Some(vec![
            ServerMessage::RoomCreated {
                room_id: room_id.clone(),
                room_type: room_type.clone(),
            },
            ServerMessage::RoomJoined {
                room_id,
                room_type,
                players: vec![],
            },
        ])
    }

    async fn handle_join_room(&self, room_id: &str) -> Option<Vec<ServerMessage>> {
        match self.room_manager.join_room(room_id, &self.client_id).await {
            Ok(_) => {
                let room = self.room_manager.get_room(room_id)?;
                let room_lock = room.read().await;

                let players = room_lock.get_players();
                let room_type = room_lock.room_type.clone();

                // Notify existing players
                // Note: In production, you'd want to actually send these to other clients
                // This would require a connection manager to store WebSocket senders

                Some(vec![ServerMessage::RoomJoined {
                    room_id: room_id.to_string(),
                    room_type,
                    players,
                }])
            }
            Err(e) => Some(vec![ServerMessage::Error { message: e }]),
        }
    }

    async fn handle_leave_room(&self) -> Option<Vec<ServerMessage>> {
        self.room_manager.leave_room(&self.client_id).await;
        None
    }

    async fn handle_send_message(&self, content: &str) -> Option<Vec<ServerMessage>> {
        let _room_id = self.room_manager.get_client_room(&self.client_id)?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_secs() as i64;

        // In production, broadcast this to all clients in the room
        Some(vec![ServerMessage::ChatMessage {
            from: self.client_id.clone(),
            content: content.to_string(),
            timestamp,
        }])
    }

    async fn handle_game_action(
        &self,
        action: &str,
        data: serde_json::Value,
    ) -> Option<Vec<ServerMessage>> {
        let _room_id = self.room_manager.get_client_room(&self.client_id)?;

        // In production, you'd update game state and broadcast to all players
        Some(vec![ServerMessage::GameAction {
            player_id: self.client_id.clone(),
            action: action.to_string(),
            data,
        }])
    }

    pub async fn cleanup(&self) {
        self.room_manager.leave_room(&self.client_id).await;
    }
}
