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

        // 3. Update AI system (future)
        // self.update_ai(&mut state);

        // 4. Update cooldowns and timers (future)
        // self.update_cooldowns(&mut state);

        // 5. Collision detection (future)
        // self.detect_collisions(&mut state);

        // 6. Combat system (future)
        // self.update_combat(&mut state);

        // 7. Generate delta updates
        // Use raw pointers to split the borrow since we know delta_tracker and world don't overlap
        let changes = unsafe {
            let world_ptr = &state.world as *const hecs::World;
            let tracker_ptr = &mut state.delta_tracker as *mut sync::DeltaTracker;
            (*tracker_ptr).update(&*world_ptr)
        };
        state.delta_sequence = state.delta_sequence.wrapping_add(1);
        let sequence = state.delta_sequence;

        // 8. Broadcast updates to clients
        let clients = self.clients.read().await;
        sync::broadcast_delta(&clients, &state, &changes, sequence).await;

        // Debug output every second
        if state.tick_count % 60 == 0 {
            debug!("Tick {}: {} players", state.tick_count, state.players.len());
        }
    }

    fn process_player_inputs(&self, state: &mut super::state::GameState) {
        // Process each player's latest input
        for (_, player_state) in state.players.iter() {
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
}
