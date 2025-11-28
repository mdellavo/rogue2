use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures::{StreamExt, SinkExt};
use log::{info, error};
use std::collections::HashMap;

use super::client::ClientConnection;
use super::messages;
use crate::game::state::SharedGameState;

pub struct GameServer {
    clients: Arc<RwLock<HashMap<u64, ClientConnection>>>,
    next_client_id: Arc<RwLock<u64>>,
    game_state: SharedGameState,
}

impl GameServer {
    pub fn new(game_state: SharedGameState) -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            next_client_id: Arc::new(RwLock::new(1)),
            game_state,
        }
    }

    pub fn get_clients(&self) -> Arc<RwLock<HashMap<u64, ClientConnection>>> {
        Arc::clone(&self.clients)
    }

    pub async fn start(&self, bind_addr: String) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&bind_addr).await?;
        info!("🌐 WebSocket server listening on: {}", bind_addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let clients = Arc::clone(&self.clients);
                    let next_id = Arc::clone(&self.next_client_id);
                    let game_state = Arc::clone(&self.game_state);

                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, addr, clients, next_id, game_state).await {
                            error!("Error handling connection from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: Arc<RwLock<HashMap<u64, ClientConnection>>>,
    next_id: Arc<RwLock<u64>>,
    game_state: SharedGameState,
) -> anyhow::Result<()> {
    info!("📥 New connection from: {}", addr);

    // Upgrade to WebSocket
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Assign client ID
    let client_id = {
        let mut id = next_id.write().await;
        let current_id = *id;
        *id += 1;
        current_id
    };

    // Create message channel for this client
    let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();

    // Store client connection
    {
        let mut clients_map = clients.write().await;
        clients_map.insert(
            client_id,
            ClientConnection {
                id: client_id,
                addr,
                sender: tx,
            },
        );
    }

    info!("✅ Client {} connected ({})", client_id, addr);

    // Task to send messages to client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = ws_sender.send(Message::Binary(msg)).await {
                error!("Error sending message to client: {}", e);
                break;
            }
        }
    });

    // Task to receive messages from client
    let clients_clone = Arc::clone(&clients);
    let game_state_clone = Arc::clone(&game_state);
    let receive_task = tokio::spawn(async move {
        while let Some(msg_result) = ws_receiver.next().await {
            match msg_result {
                Ok(Message::Binary(data)) => {
                    info!("📨 Received {} bytes from client {}", data.len(), client_id);
                    // Parse and handle FlatBuffers message
                    messages::handle_message(&data, client_id, Arc::clone(&game_state_clone), Arc::clone(&clients_clone)).await;
                }
                Ok(Message::Close(_)) => {
                    info!("🔌 Client {} requested close", client_id);
                    break;
                }
                Ok(Message::Ping(_data)) => {
                    // WebSocket library handles Pong automatically
                    info!("🏓 Ping from client {}", client_id);
                }
                Err(e) => {
                    error!("WebSocket error from client {}: {}", client_id, e);
                    break;
                }
                _ => {}
            }
        }

        // Remove client on disconnect
        let mut clients_map = clients_clone.write().await;
        if clients_map.remove(&client_id).is_some() {
            info!("👋 Client {} disconnected", client_id);
        }

        // Remove player from game state
        let mut state = game_state.write().await;
        state.remove_player(client_id);
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = receive_task => {},
    }

    Ok(())
}
