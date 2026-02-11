use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Messages sent from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    // Room management
    CreateRoom {
        room_type: RoomType,
        max_players: Option<usize>,
    },
    JoinRoom {
        room_id: String,
    },
    LeaveRoom,

    // Chat
    SendMessage {
        content: String,
    },

    // Game actions (generic untuk berbagai game)
    GameAction {
        action: String,
        data: serde_json::Value,
    },
}

/// Messages sent from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Connected {
        client_id: String,
    },
    RoomCreated {
        room_id: String,
        room_type: RoomType,
    },
    RoomJoined {
        room_id: String,
        room_type: RoomType,
        players: Vec<Player>,
    },
    PlayerJoined {
        player: Player,
    },
    PlayerLeft {
        client_id: String,
    },
    ChatMessage {
        from: String,
        content: String,
        timestamp: i64,
    },
    GameState {
        state: serde_json::Value,
    },
    GameAction {
        player_id: String,
        action: String,
        data: serde_json::Value,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomType {
    Chat,
    CardGame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: Option<String>,
    pub ready: bool,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub room_type: RoomType,
    pub players: HashMap<String, Player>,
    pub max_players: usize,
    pub game_state: Option<serde_json::Value>,
}

impl Room {
    pub fn new(id: String, room_type: RoomType, max_players: usize) -> Self {
        Self {
            id,
            room_type,
            players: HashMap::new(),
            max_players,
            game_state: None,
        }
    }

    pub fn add_player(&mut self, client_id: String) -> bool {
        if self.players.len() >= self.max_players {
            return false;
        }

        let player = Player {
            id: client_id.clone(),
            name: None,
            ready: false,
        };

        self.players.insert(client_id, player);
        true
    }

    pub fn remove_player(&mut self, client_id: &str) -> bool {
        self.players.remove(client_id).is_some()
    }

    pub fn get_players(&self) -> Vec<Player> {
        self.players.values().cloned().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }
}
