use crate::types::{Room, RoomType};
use dashmap::DashMap;
use std::sync::Arc;

pub struct RoomManager {
    rooms: DashMap<String, Arc<tokio::sync::RwLock<Room>>>,
    client_rooms: DashMap<String, String>, // client_id -> room_id
}

impl Default for RoomManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: DashMap::new(),
            client_rooms: DashMap::new(),
        }
    }

    pub fn create_room(&self, room_type: RoomType, max_players: Option<usize>) -> String {
        let room_id = uuid::Uuid::new_v4().to_string();
        let max_players = max_players.unwrap_or(match room_type {
            RoomType::Chat => 50,
            RoomType::CardGame => 4,
        });

        let room = Room::new(room_id.clone(), room_type, max_players);
        self.rooms
            .insert(room_id.clone(), Arc::new(tokio::sync::RwLock::new(room)));

        room_id
    }

    pub async fn join_room(&self, room_id: &str, client_id: &str) -> Result<(), String> {
        // Check if client is already in a room
        if self.client_rooms.contains_key(client_id) {
            return Err("Already in a room".to_string());
        }

        let room = self
            .rooms
            .get(room_id)
            .ok_or_else(|| "Room not found".to_string())?;

        let mut room_lock = room.write().await;

        if !room_lock.add_player(client_id.to_string()) {
            return Err("Room is full".to_string());
        }

        self.client_rooms
            .insert(client_id.to_string(), room_id.to_string());

        Ok(())
    }

    pub async fn leave_room(&self, client_id: &str) -> Option<String> {
        let room_id = self.client_rooms.remove(client_id)?.1;

        if let Some(room) = self.rooms.get(&room_id) {
            let mut room_lock = room.write().await;
            room_lock.remove_player(client_id);

            // Remove empty rooms
            if room_lock.is_empty() {
                drop(room_lock);
                self.rooms.remove(&room_id);
            }
        }

        Some(room_id)
    }

    pub fn get_room(&self, room_id: &str) -> Option<Arc<tokio::sync::RwLock<Room>>> {
        self.rooms.get(room_id).map(|r| r.clone())
    }

    pub fn get_client_room(&self, client_id: &str) -> Option<String> {
        self.client_rooms.get(client_id).map(|r| r.clone())
    }

    pub async fn broadcast_to_room<F>(
        &self,
        room_id: &str,
        exclude_client: Option<&str>,
        mut callback: F,
    ) where
        F: FnMut(&str),
    {
        if let Some(room) = self.rooms.get(room_id) {
            let room_lock = room.read().await;
            for client_id in room_lock.players.keys() {
                if Some(client_id.as_str()) != exclude_client {
                    callback(client_id);
                }
            }
        }
    }
}
