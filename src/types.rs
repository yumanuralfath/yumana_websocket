use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub profile_picture_url: Option<String>,
}

#[derive(Deserialize)]
pub struct ApiResponse {
    pub user: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Authenticate {
        token: String,
    },
    CreateRoom {
        room_type: RoomType,
        max_players: Option<usize>,
    },
    JoinRoom {
        room_id: String,
    },
    LeaveRoom,
    SendMessage {
        content: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Authenticated {
        user: UserProfile,
    },
    RoomJoined {
        room_id: String,
        players: Vec<Player>,
    },
    ChatMessage {
        from: String,
        content: String,
        timestamp: i64,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomType {
    Chat,
    CardGame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub players: HashMap<String, Player>,
    pub max_players: usize,
}

impl Room {
    pub fn new(id: String, max_players: usize) -> Self {
        Self {
            id,
            players: HashMap::new(),
            max_players,
        }
    }
}
