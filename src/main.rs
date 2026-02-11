use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info, warn};

use yumana_websocket::*;

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("WebSocket server listening on: {}", addr);

    let room_manager = Arc::new(RoomManager::new());

    while let Ok((stream, peer_addr)) = listener.accept().await {
        info!("New connection from: {}", peer_addr);
        let room_manager = Arc::clone(&room_manager);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, room_manager).await {
                error!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    room_manager: Arc<RoomManager>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let client_id = uuid::Uuid::new_v4().to_string();
    let handler = MessageHandler::new(client_id.clone(), room_manager);

    info!("Client {} connected", client_id);

    // Send welcome message
    let welcome = ServerMessage::Connected {
        client_id: client_id.clone(),
    };
    ws_sender
        .send(Message::Text(serde_json::to_string(&welcome)?))
        .await?;

    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(client_msg) => {
                    if let Some(responses) = handler.handle_message(client_msg).await {
                        for response in responses {
                            let json = serde_json::to_string(&response)?;
                            ws_sender.send(Message::Text(json)).await?;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to parse message: {}", e);
                    let error = ServerMessage::Error {
                        message: "Invalid message format".to_string(),
                    };
                    ws_sender
                        .send(Message::Text(serde_json::to_string(&error)?))
                        .await?;
                }
            },
            Ok(Message::Close(_)) => {
                info!("Client {} disconnected", client_id);
                handler.cleanup().await;
                break;
            }
            Ok(Message::Ping(data)) => {
                ws_sender.send(Message::Pong(data)).await?;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
