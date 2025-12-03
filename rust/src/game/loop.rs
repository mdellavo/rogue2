use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::sleep;
use log::{info, debug};

use super::state::SharedGameState;
use super::sync;
use crate::ecs::components::*;
use crate::network::client::ClientConnection;

const TICK_RATE: u64 = 60; // 60 Hz
const TICK_DURATION_MS: u64 = 1000 / TICK_RATE; // ~16.67ms
const MOVEMENT_SPEED: f32 = 200.0; // pixels per second
const TICK_DURATION_SEC: f32 = 1.0 / TICK_RATE as f32;

pub struct GameLoop {
    state: SharedGameState,
    clients: Arc<RwLock<HashMap<u64, ClientConnection>>>,
}

impl GameLoop {
    pub fn new(state: SharedGameState, clients: Arc<RwLock<HashMap<u64, ClientConnection>>>) -> Self {
        Self { state, clients }
    }

    pub async fn run(&self) {
        info!("🎮 Game loop starting at {} Hz", TICK_RATE);

        let tick_duration = Duration::from_millis(TICK_DURATION_MS);
        let mut tick_start: Instant;

        loop {
            tick_start = Instant::now();

            // Run game tick
            self.tick().await;

            // Sleep until next tick
            let elapsed = tick_start.elapsed();
            if elapsed < tick_duration {
                sleep(tick_duration - elapsed).await;
            } else {
                debug!("Tick took {}ms (target: {}ms)", elapsed.as_millis(), TICK_DURATION_MS);
            }
        }
    }

    async fn tick(&self) {
        let mut state = self.state.write().await;

        // Increment tick counter
        state.tick_count += 1;

        // 1. Process player inputs (movement)
        self.process_player_inputs(&mut state);

        // 2. Update movement system (apply velocity to position)
        self.update_movement(&mut state);

        // 3. Check for chunk updates (players moving between chunks)
        self.update_player_chunks(&mut state).await;

        // 4. Update AI system (future)
        // self.update_ai(&mut state);

        // 5. Update cooldowns and timers (future)
        // self.update_cooldowns(&mut state);

        // 6. Collision detection (future)
        // self.detect_collisions(&mut state);

        // 7. Combat system (future)
        // self.update_combat(&mut state);

        // 8. Generate delta updates
        // Use raw pointers to split the borrow since we know delta_tracker and world don't overlap
        let changes = unsafe {
            let world_ptr = &state.world as *const hecs::World;
            let tracker_ptr = &mut state.delta_tracker as *mut sync::DeltaTracker;
            (*tracker_ptr).update(&*world_ptr)
        };
        state.delta_sequence = state.delta_sequence.wrapping_add(1);
        let sequence = state.delta_sequence;

        // 9. Broadcast updates to clients
        let clients = self.clients.read().await;
        sync::broadcast_delta(&clients, &state, &changes, sequence).await;

        // Debug output every second
        if state.tick_count % 60 == 0 {
            debug!("Tick {}: {} players", state.tick_count, state.players.len());
        }
    }

    fn process_player_inputs(&self, state: &mut super::state::GameState) {
        // Process each player's latest input
        for (client_id, player_state) in state.players.iter() {
            let input = &player_state.latest_input;

            // Update velocity based on input
            if let Ok(mut velocity) = state.world.get::<&mut Velocity>(player_state.entity) {
                // Normalize diagonal movement
                let input_length = (input.movement_x * input.movement_x + input.movement_y * input.movement_y).sqrt();

                if input_length > 0.0 {
                    let normalized_x = input.movement_x / input_length;
                    let normalized_y = input.movement_y / input_length;

                    velocity.dx = normalized_x * MOVEMENT_SPEED;
                    velocity.dy = normalized_y * MOVEMENT_SPEED;

                    // Debug: Log when applying movement
                    if state.tick_count % 60 == 0 {
                        debug!("Player {} velocity set to ({:.2}, {:.2})", client_id, velocity.dx, velocity.dy);
                    }
                } else {
                    velocity.dx = 0.0;
                    velocity.dy = 0.0;
                }
            }
        }
    }

    fn update_movement(&self, state: &mut super::state::GameState) {
        // Apply velocity to position for all entities with both components
        for (_entity, (pos, vel)) in state.world.query_mut::<(&mut Position, &Velocity)>() {
            pos.x += vel.dx * TICK_DURATION_SEC;
            pos.y += vel.dy * TICK_DURATION_SEC;

            // Simple boundary checking (TODO: use actual map bounds)
            const MAP_MIN: f32 = 0.0;
            const MAP_MAX: f32 = 3200.0; // 100 tiles * 32 pixels

            pos.x = pos.x.clamp(MAP_MIN, MAP_MAX);
            pos.y = pos.y.clamp(MAP_MIN, MAP_MAX);
        }
    }

    async fn update_player_chunks(&self, state: &mut super::state::GameState) {
        use crate::generated::messages_generated::game::network;

        let clients = self.clients.read().await;

        // Check each player for chunk updates
        let player_ids: Vec<u64> = state.players.keys().copied().collect();
        for player_id in player_ids {
            let chunk_update = state.update_player_chunks(player_id);

            // Only send messages if there are changes
            if !chunk_update.to_load.is_empty() || !chunk_update.to_unload.is_empty() {
                if let Some(client) = clients.get(&player_id) {
                    // Send ChunksLoaded for new chunks
                    if !chunk_update.to_load.is_empty() {
                        let mut builder = flatbuffers::FlatBufferBuilder::new();

                        let mut chunk_offsets = Vec::new();
                        for (cx, cy) in &chunk_update.to_load {
                            if let Some(chunk) = state.chunk_system.get_chunk(*cx, *cy) {
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
                            debug!("Failed to send ChunksLoaded to client {}: {}", player_id, e);
                        } else {
                            debug!("📤 Sent {} new chunks to client {}", chunk_update.to_load.len(), player_id);
                        }
                    }

                    // Send ChunksUnloaded for chunks to remove
                    if !chunk_update.to_unload.is_empty() {
                        let mut builder = flatbuffers::FlatBufferBuilder::new();

                        let chunk_coords: Vec<_> = chunk_update.to_unload.iter().map(|(cx, cy)| {
                            network::ChunkCoord::create(&mut builder, &network::ChunkCoordArgs {
                                x: *cx,
                                y: *cy,
                            })
                        }).collect();

                        let coords_vector = builder.create_vector(&chunk_coords);
                        let chunks_unloaded = network::ChunksUnloaded::create(&mut builder, &network::ChunksUnloadedArgs {
                            chunk_coords: Some(coords_vector),
                        });

                        let message = network::Message::create(&mut builder, &network::MessageArgs {
                            payload_type: network::MessageType::ChunksUnloaded,
                            payload: Some(chunks_unloaded.as_union_value()),
                        });

                        builder.finish(message, None);
                        let data = builder.finished_data().to_vec();

                        if let Err(e) = client.send_message(data) {
                            debug!("Failed to send ChunksUnloaded to client {}: {}", player_id, e);
                        } else {
                            debug!("📤 Sent {} unload requests to client {}", chunk_update.to_unload.len(), player_id);
                        }
                    }
                }
            }
        }
    }
}
