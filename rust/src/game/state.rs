use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use hecs::{World, Entity};

use crate::ecs::components::*;
use crate::game::sync::DeltaTracker;
use crate::map::types::MapData;
use crate::map::chunks::{ChunkSystem, calculate_chunk_update, ChunkUpdate};

fn species_sprite_id(species: Species) -> &'static str {
    match species {
        Species::Human => "human",
        Species::Elf => "elf",
        Species::Dwarf => "dwarf",
        Species::Halfling => "halfling",
        Species::HalfOrc => "halforc",
        Species::Gnome => "gnome",
    }
}

fn class_sprite_id(class: CharacterClass) -> &'static str {
    match class {
        CharacterClass::Fighter => "fighter",
        CharacterClass::Rogue => "rogue",
        CharacterClass::Cleric => "cleric",
        CharacterClass::Wizard => "wizard",
        CharacterClass::Ranger => "ranger",
        CharacterClass::Barbarian => "barbarian",
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerInput {
    pub sequence: u32,
    pub timestamp: u64,
    pub movement_x: f32,
    pub movement_y: f32,
    pub action: u8, // 0=none, 1=attack, 2=interact
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            sequence: 0,
            timestamp: 0,
            movement_x: 0.0,
            movement_y: 0.0,
            action: 0,
        }
    }
}

pub struct PlayerState {
    pub entity: Entity,
    pub latest_input: PlayerInput,
    pub loaded_chunks: HashSet<(i32, i32)>, // Chunks currently loaded for this player
}

pub struct GameState {
    pub world: World,
    pub players: HashMap<u64, PlayerState>, // connection_id -> PlayerState
    pub tick_count: u64,
    pub delta_tracker: DeltaTracker,
    pub delta_sequence: u32,
    pub map: MapData,
    pub chunk_system: ChunkSystem, // Chunk system for efficient map streaming
    pub next_spawn_index: usize, // Round-robin spawn point selection
}

impl GameState {
    pub fn new(map: MapData) -> Self {
        let chunk_system = ChunkSystem::from_map_data(&map);

        Self {
            world: World::new(),
            players: HashMap::new(),
            tick_count: 0,
            delta_tracker: DeltaTracker::new(),
            delta_sequence: 0,
            map,
            chunk_system,
            next_spawn_index: 0,
        }
    }

    /// Get the next spawn point in round-robin fashion
    pub fn get_next_spawn_point(&mut self) -> (f32, f32) {
        if self.map.spawn_points.is_empty() {
            // Fallback to center if no spawn points defined
            return (1600.0, 1600.0);
        }

        let spawn_point = &self.map.spawn_points[self.next_spawn_index];
        let position = (spawn_point.x, spawn_point.y);

        // Move to next spawn point for next player
        self.next_spawn_index = (self.next_spawn_index + 1) % self.map.spawn_points.len();

        position
    }

    pub fn add_player(
        &mut self,
        connection_id: u64,
        name: String,
        species: Species,
        class: CharacterClass,
        spawn_x: f32,
        spawn_y: f32,
    ) -> Entity {
        // Create character using character builder
        let character_sheet = super::character::CharacterBuilder::new(species, class).build();

        let entity = self.world.spawn((
            Position { x: spawn_x, y: spawn_y },
            Velocity { dx: 0.0, dy: 0.0 },
            Health {
                current: character_sheet.max_hp,
                max: character_sheet.max_hp,
            },
            character_sheet.stats,
            Character {
                species: character_sheet.species,
                class: character_sheet.class,
                level: character_sheet.level,
                experience: character_sheet.experience,
            },
            Player {
                name,
                connection_id,
            },
            Sprite {
                sprite_id: format!("{}_{}", species_sprite_id(species), class_sprite_id(class)),
            },
            Collider { radius: 16.0 },
            ArmorClass { value: character_sheet.ac },
            MovementSpeed { pixels_per_second: character_sheet.movement_speed },
            VisionRange { tiles: character_sheet.vision_range },
            AttackSpeed { cooldown_ms: character_sheet.attack_cooldown_ms },
            Cooldowns::default(),
        ));

        self.players.insert(connection_id, PlayerState {
            entity,
            latest_input: PlayerInput::default(),
            loaded_chunks: HashSet::new(), // Start with no chunks loaded
        });

        // Mark entity as newly spawned for delta tracking
        self.delta_tracker.mark_spawned(entity);

        entity
    }

    pub fn remove_player(&mut self, connection_id: u64) {
        if let Some(player_state) = self.players.remove(&connection_id) {
            // Mark entity as despawned for delta tracking
            self.delta_tracker.mark_despawned(player_state.entity);
            let _ = self.world.despawn(player_state.entity);
        }
    }

    pub fn update_player_input(&mut self, connection_id: u64, input: PlayerInput) {
        if let Some(player_state) = self.players.get_mut(&connection_id) {
            player_state.latest_input = input;
        }
    }

    pub fn get_player_entity(&self, connection_id: u64) -> Option<Entity> {
        self.players.get(&connection_id).map(|ps| ps.entity)
    }

    /// Check if chunks need updating for a player and return what changed
    pub fn update_player_chunks(&mut self, connection_id: u64) -> ChunkUpdate {
        let player_state = match self.players.get(&connection_id) {
            Some(state) => state,
            None => return ChunkUpdate::default(),
        };

        // Get player position
        let position = match self.world.get::<&Position>(player_state.entity) {
            Ok(pos) => (pos.x, pos.y),
            Err(_) => return ChunkUpdate::default(),
        };

        // Get chunks needed for this position
        let needed_chunks = self.chunk_system.get_chunks_for_position(position.0, position.1);
        let needed_set: HashSet<_> = needed_chunks.into_iter().collect();

        // Calculate chunks to load and unload
        let player_state = self.players.get(&connection_id).unwrap();
        let update = calculate_chunk_update(&player_state.loaded_chunks, &needed_set);

        // Update loaded chunks
        let player_state = self.players.get_mut(&connection_id).unwrap();
        player_state.loaded_chunks = needed_set;

        update
    }
}

pub type SharedGameState = Arc<RwLock<GameState>>;
