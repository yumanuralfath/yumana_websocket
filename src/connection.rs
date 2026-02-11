use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

pub struct ConnectionManager {
    // user_id -> sender channel
    pub connections: DashMap<String, mpsc::UnboundedSender<Message>>,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: DashMap::new(),
        }
    }

    pub fn add(&self, id: String, tx: mpsc::UnboundedSender<Message>) {
        self.connections.insert(id, tx);
    }

    pub fn remove(&self, id: &str) {
        self.connections.remove(id);
    }

    pub fn send_to(&self, id: &str, msg: Message) {
        if let Some(tx) = self.connections.get(id) {
            let _ = tx.send(msg);
        }
    }
}
