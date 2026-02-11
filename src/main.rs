use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info};

use yumana_websocket::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or("8080".into());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("Server WebSocket running at: {}", addr);

    let conn_manager = Arc::new(ConnectionManager::new());
    let room_manager = Arc::new(RoomManager::new(conn_manager.clone()));

    while let Ok((stream, peer_addr)) = listener.accept().await {
        let rm = room_manager.clone();
        let cm = conn_manager.clone();

        tokio::spawn(async move {
            info!("Koneksi baru dari: {}", peer_addr);
            handle_connection(stream, rm, cm).await;
        });
    }
}

async fn validate_with_api(token: &str) -> Option<UserProfile> {
    let client = reqwest::Client::new();
    // Endpoint API sesuai data Anda
    let res = client
        .get("https://api.yumana.my.id/me")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .ok()?;

    if res.status().is_success() {
        let body = res.text().await.ok()?;
        // Menangani jika JSON dibungkus key "user" atau langsung object
        if let Ok(wrapper) = serde_json::from_str::<ApiResponse>(&body) {
            Some(wrapper.user)
        } else {
            serde_json::from_str::<UserProfile>(&body).ok()
        }
    } else {
        None
    }
}

async fn handle_connection(stream: TcpStream, rm: Arc<RoomManager>, cm: Arc<ConnectionManager>) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Handshake gagal: {}", e);
            return;
        }
    };

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Task untuk mengirim pesan ke client
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if (ws_sender.send(msg).await).is_err() {
                break;
            }
        }
    });

    let mut auth_user: Option<UserProfile> = None;

    while let Some(msg) = ws_receiver.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(_) => break,
        };

        if let Message::Text(text) = msg {
            if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                match client_msg {
                    ClientMessage::Authenticate { token } => {
                        if let Some(user) = validate_with_api(&token).await {
                            let id_str = user.id.to_string();
                            let username = user.username.clone();

                            auth_user = Some(user.clone());
                            cm.add(id_str.clone(), tx.clone());

                            // OTOMATIS MASUKKAN KE LOBBY
                            let player = Player {
                                id: id_str.clone(),
                                username: username.clone(),
                            };
                            if !rm.rooms.contains_key("lobby") {
                                rm.rooms.insert(
                                    "lobby".to_string(),
                                    Arc::new(tokio::sync::RwLock::new(Room::new(
                                        "lobby".to_string(),
                                        100,
                                    ))),
                                );
                            }
                            let _ = rm.join_room("lobby", player).await;

                            let _ = tx.send(Message::Text(
                                serde_json::to_string(&ServerMessage::Authenticated { user })
                                    .unwrap(),
                            ));
                            info!(
                                "User {} ({}) terautentikasi dan masuk lobby",
                                username, id_str
                            );
                        } else {
                            let _ = tx.send(Message::Text(
                                serde_json::to_string(&ServerMessage::Error {
                                    message: "Auth Gagal".into(),
                                })
                                .unwrap(),
                            ));
                        }
                    }
                    other => {
                        if let Some(user) = &auth_user {
                            let handler = MessageHandler {
                                user_id: user.id.to_string(),
                                username: user.username.clone(),
                                room_manager: rm.clone(),
                            };
                            handler.handle(other).await;
                        }
                    }
                }
            }
        }
    }

    if let Some(user) = auth_user {
        info!("User {} disconnected", user.username);
        cm.remove(&user.id.to_string());
    }
}
