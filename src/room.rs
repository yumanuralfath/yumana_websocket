use crate::connection::ConnectionManager;
use crate::types::{Player, Room};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

pub struct RoomManager {
    pub rooms: DashMap<String, Arc<RwLock<Room>>>,
    pub client_rooms: DashMap<String, String>,
    pub conn_manager: Arc<ConnectionManager>,
}

impl RoomManager {
    pub fn new(conn_manager: Arc<ConnectionManager>) -> Self {
        Self {
            rooms: DashMap::new(),
            client_rooms: DashMap::new(),
            conn_manager,
        }
    }

    pub async fn join_room(&self, room_id: &str, player: Player) -> Result<(), String> {
        let room = self.rooms.get(room_id).ok_or("Room not found")?;
        let player_id = player.id.clone(); // simpan dulu
        let mut lock = room.write().await;
        lock.players.insert(player_id.clone(), player);
        self.client_rooms.insert(player_id, room_id.to_string());

        Ok(())
    }

    pub async fn broadcast(&self, room_id: &str, msg: Message) {
        if let Some(room) = self.rooms.get(room_id) {
            let lock = room.read().await;
            for player_id in lock.players.keys() {
                self.conn_manager.send_to(player_id, msg.clone());
            }
        }
    }
}
