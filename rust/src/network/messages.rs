use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, warn, debug};
use crate::game::state::{SharedGameState, PlayerInput as GamePlayerInput};
use crate::game::sync;
use crate::generated::messages_generated::game::network;
use crate::ecs::components::{Species, CharacterClass};
use crate::network::client::ClientConnection;

// Type aliases for convenience
type Message<'a> = network::Message<'a>;
type MessageType = network::MessageType;
type PlayerInput<'a> = network::PlayerInput<'a>;
type PlayerJoin<'a> = network::PlayerJoin<'a>;
type Ping<'a> = network::Ping<'a>;
type ChatMessage<'a> = network::ChatMessage<'a>;
type InteractDoor<'a> = network::InteractDoor<'a>;
type RequestChunks<'a> = network::RequestChunks<'a>;

pub async fn handle_message(
    data: &[u8],
    client_id: u64,
    game_state: SharedGameState,
    clients: Arc<RwLock<HashMap<u64, ClientConnection>>>,
) {
    // Parse the FlatBuffers message
    let message = match flatbuffers::root::<Message>(data) {
        Ok(msg) => msg,
        Err(e) => {
            warn!("Failed to parse message from client {}: {}", client_id, e);
            return;
        }
    };

    // Get the message type
    let msg_type = message.payload_type();
    debug!("Client {} sent message type: {:?}", client_id, msg_type);

    // Handle different message types
    match msg_type {
        MessageType::PlayerJoin => {
            if let Some(join) = message.payload_as_player_join() {
                handle_player_join(join, client_id, game_state, clients).await;
            } else {
                warn!("Client {} sent PlayerJoin but payload is invalid", client_id);
            }
        }
        MessageType::PlayerInput => {
            if let Some(player_input) = message.payload_as_player_input() {
                handle_player_input(player_input, client_id, game_state).await;
            } else {
                warn!("Client {} sent PlayerInput but payload is invalid", client_id);
            }
        }
        MessageType::Ping => {
            if let Some(ping) = message.payload_as_ping() {
                handle_ping(ping, client_id).await;
            }
        }
        MessageType::ChatMessage => {
            if let Some(chat) = message.payload_as_chat_message() {
                handle_chat_message(chat, client_id).await;
            }
        }
        MessageType::InteractDoor => {
            if let Some(interact) = message.payload_as_interact_door() {
                handle_interact_door(interact, client_id).await;
            }
        }
        MessageType::RequestChunks => {
            if let Some(request) = message.payload_as_request_chunks() {
                handle_request_chunks(request, client_id, game_state, clients).await;
            }
        }
        _ => {
            warn!("Client {} sent unhandled message type: {:?}", client_id, msg_type);
        }
    }
}

async fn handle_player_input(input: PlayerInput<'_>, client_id: u64, game_state: SharedGameState) {
    let movement = input.movement();
    let sequence = input.sequence();
    let timestamp = input.timestamp();
    let action = input.action();

    debug!(
        "Player {} input: seq={}, movement=({:.2}, {:.2}), action={}",
        client_id,
        sequence,
        movement.map(|v| v.x()).unwrap_or(0.0),
        movement.map(|v| v.y()).unwrap_or(0.0),
        action
    );

    // Update player input in game state
    let game_input = GamePlayerInput {
        sequence,
        timestamp,
        movement_x: movement.map(|v| v.x()).unwrap_or(0.0),
        movement_y: movement.map(|v| v.y()).unwrap_or(0.0),
        action,
    };

    let mut state = game_state.write().await;
    state.update_player_input(client_id, game_input);
}

async fn handle_ping(_ping: Ping<'_>, client_id: u64) {
    debug!("Received ping from client {}", client_id);
    // TODO: Send Pong response
}

async fn handle_chat_message(chat: ChatMessage<'_>, client_id: u64) {
    if let Some(message) = chat.message() {
        info!("Chat from client {}: {}", client_id, message);
        // TODO: Broadcast to nearby players
    }
}

async fn handle_interact_door(interact: InteractDoor<'_>, client_id: u64) {
    let door_id = interact.door_entity_id();
    info!("Client {} interacting with door {}", client_id, door_id);
    // TODO: Handle door interaction
}

async fn handle_player_join(
    join: PlayerJoin<'_>,
    client_id: u64,
    game_state: SharedGameState,
    clients: Arc<RwLock<HashMap<u64, ClientConnection>>>,
) {
    let name = join.name().unwrap_or("Unknown").to_string();
    let species_value = join.species();
    let class_value = join.character_class();

    // Convert FlatBuffers enums to our ECS enums
    let species = match species_value {
        network::Species::Human => Species::Human,
        network::Species::Elf => Species::Elf,
        network::Species::Dwarf => Species::Dwarf,
        network::Species::Halfling => Species::Halfling,
        network::Species::HalfOrc => Species::HalfOrc,
        network::Species::Gnome => Species::Gnome,
        _ => {
            warn!("Client {} sent invalid species: {:?}", client_id, species_value);
            return;
        }
    };

    let class = match class_value {
        network::CharacterClass::Fighter => CharacterClass::Fighter,
        network::CharacterClass::Rogue => CharacterClass::Rogue,
        network::CharacterClass::Cleric => CharacterClass::Cleric,
        network::CharacterClass::Wizard => CharacterClass::Wizard,
        network::CharacterClass::Ranger => CharacterClass::Ranger,
        network::CharacterClass::Barbarian => CharacterClass::Barbarian,
        _ => {
            warn!("Client {} sent invalid class: {:?}", client_id, class_value);
            return;
        }
    };

    info!("Client {} joining as {} {:?} {:?}", client_id, name, species, class);

    // Get next spawn point from map
    let entity = {
        let mut state = game_state.write().await;
        let (spawn_x, spawn_y) = state.get_next_spawn_point();
        info!("  Spawning at ({}, {})", spawn_x, spawn_y);
        state.add_player(client_id, name, species, class, spawn_x, spawn_y)
    };

    info!("✅ Player {} spawned successfully", client_id);

    // Send GameStateSnapshot to the newly connected client
    let clients_map = clients.read().await;
    if let Some(client) = clients_map.get(&client_id) {
        let state = game_state.read().await;
        sync::send_snapshot_to_client(client, &state, entity).await;
        info!("📤 Sent initial snapshot to client {}", client_id);
    } else {
        warn!("Client {} not found in clients map after join", client_id);
    }
}

async fn handle_request_chunks(
    request: RequestChunks<'_>,
    client_id: u64,
    game_state: SharedGameState,
    clients: Arc<RwLock<HashMap<u64, ClientConnection>>>,
) {
    let chunk_coords = match request.chunk_coords() {
        Some(coords) => coords,
        None => {
            warn!("Client {} requested chunks but provided no coordinates", client_id);
            return;
        }
    };

    debug!("Client {} requesting {} chunks", client_id, chunk_coords.len());

    let state = game_state.read().await;
    let clients_map = clients.read().await;

    if let Some(client) = clients_map.get(&client_id) {
        // Build FlatBuffers message with requested chunks
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let mut chunk_offsets = Vec::new();
        for coord in chunk_coords {
            let cx = coord.x();
            let cy = coord.y();

            if let Some(chunk) = state.chunk_system.get_chunk(cx, cy) {
                let tiles = builder.create_vector(&chunk.tiles);

                let mut feature_offsets = Vec::new();
                for f in &chunk.features {
                    feature_offsets.push(network::ChunkFeature::create(&mut builder, &network::ChunkFeatureArgs {
                        tile_x: f.tile_x,
                        tile_y: f.tile_y,
                        feature_id: f.feature_id,
                    }));
                }
                let features_vec = builder.create_vector(&feature_offsets);

                let chunk_offset = network::ChunkData::create(&mut builder, &network::ChunkDataArgs {
                    chunk_x: chunk.chunk_x,
                    chunk_y: chunk.chunk_y,
                    tiles: Some(tiles),
                    features: Some(features_vec),
                });
                chunk_offsets.push(chunk_offset);
            } else {
                warn!("Client {} requested invalid chunk ({}, {})", client_id, cx, cy);
            }
        }

        let chunks_vector = builder.create_vector(&chunk_offsets);
        let chunks_loaded = network::ChunksLoaded::create(&mut builder, &network::ChunksLoadedArgs {
            chunks: Some(chunks_vector),
        });

        let message = network::Message::create(&mut builder, &network::MessageArgs {
            payload_type: network::MessageType::ChunksLoaded,
            payload: Some(chunks_loaded.as_union_value()),
        });

        builder.finish(message, None);
        let data = builder.finished_data().to_vec();

        if let Err(e) = client.send_message(data) {
            warn!("Failed to send chunks to client {}: {}", client_id, e);
        } else {
            debug!("📤 Sent {} chunks to client {}", chunk_offsets.len(), client_id);
        }
    } else {
        warn!("Client {} not found when trying to send chunks", client_id);
    }
}
