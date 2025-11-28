use std::collections::{HashMap, HashSet};
use hecs::Entity;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use log::{debug, warn};

use crate::ecs::components::*;
use crate::game::state::GameState;
use crate::generated::messages_generated::game::network::{
    self, EntityData, EntityDataArgs, GameStateDelta, GameStateDeltaArgs,
    GameStateSnapshot, GameStateSnapshotArgs, Message, MessageArgs, MessageType,
    Stats as FbStats, Vec2, Vec2Args,
};
use crate::network::client::ClientConnection;

const VISION_RANGE_PIXELS: f32 = 640.0; // 20 tiles * 32 pixels

/// Helper function to update delta tracker without simultaneous borrows
pub fn update_delta_tracker(delta_tracker: &mut DeltaTracker, world: &hecs::World) -> EntityChanges {
    delta_tracker.update(world)
}

/// Tracks which entities have changed since last update
pub struct DeltaTracker {
    // Track entity positions from last tick to detect movement
    last_positions: HashMap<Entity, (f32, f32)>,
    // Track entities that were just spawned
    newly_spawned: HashSet<Entity>,
    // Track entities that were despawned
    despawned: HashSet<u32>, // Using u32 for entity ID
}

impl DeltaTracker {
    pub fn new() -> Self {
        Self {
            last_positions: HashMap::new(),
            newly_spawned: HashSet::new(),
            despawned: HashSet::new(),
        }
    }

    /// Mark an entity as newly spawned
    pub fn mark_spawned(&mut self, entity: Entity) {
        self.newly_spawned.insert(entity);
    }

    /// Mark an entity as despawned
    pub fn mark_despawned(&mut self, entity: Entity) {
        let entity_id = entity.to_bits().get() as u32;
        self.despawned.insert(entity_id);
        self.last_positions.remove(&entity);
    }

    /// Update tracking for all entities and detect changes
    pub fn update(&mut self, world: &hecs::World) -> EntityChanges {
        let mut updated = HashSet::new();
        let mut current_positions = HashMap::new();

        // Check all entities with Position component for changes
        for (entity, position) in world.query::<&Position>().iter() {
            let current_pos = (position.x, position.y);
            current_positions.insert(entity, current_pos);

            // Check if position changed
            if let Some(last_pos) = self.last_positions.get(&entity) {
                if (current_pos.0 - last_pos.0).abs() > 0.01
                    || (current_pos.1 - last_pos.1).abs() > 0.01
                {
                    updated.insert(entity);
                }
            }
        }

        // Update last positions
        self.last_positions = current_positions;

        let changes = EntityChanges {
            spawned: self.newly_spawned.drain().collect(),
            updated,
            despawned: self.despawned.drain().collect(),
        };

        changes
    }
}

pub struct EntityChanges {
    pub spawned: Vec<Entity>,
    pub updated: HashSet<Entity>,
    pub despawned: Vec<u32>,
}

// Helper struct to hold entity data before building FlatBuffers
struct EntityInfo {
    id: u32,
    pos: (f32, f32),
    vel: (f32, f32),
    sprite_id: String,
    health_current: i32,
    health_max: i32,
    stats: Stats,
    name: String,
    species: Species,
    class: CharacterClass,
    level: u32,
    experience: u32,
}

/// Collect entity info from the world
fn collect_entity_info(world: &hecs::World, entity: Entity) -> Option<EntityInfo> {
    let entity_id = entity.to_bits().get() as u32;

    // Get required components
    let position = world.get::<&Position>(entity).ok()?;
    let velocity = world.get::<&Velocity>(entity).ok()?;
    let health = world.get::<&Health>(entity).ok()?;
    let stats = world.get::<&Stats>(entity).ok()?;

    // Get optional components
    let sprite_id = if let Ok(sprite) = world.get::<&Sprite>(entity) {
        sprite.sprite_id.clone()
    } else {
        "default".to_string()
    };

    let name = if let Ok(player) = world.get::<&Player>(entity) {
        player.name.clone()
    } else {
        "NPC".to_string()
    };

    // Get species and class (default to Human/Fighter for NPCs)
    let (species, class, level, experience) =
        if let Ok(character) = world.get::<&Character>(entity) {
            (character.species, character.class, character.level, character.experience)
        } else {
            (Species::Human, CharacterClass::Fighter, 1, 0)
        };

    Some(EntityInfo {
        id: entity_id,
        pos: (position.x, position.y),
        vel: (velocity.dx, velocity.dy),
        sprite_id,
        health_current: health.current,
        health_max: health.max,
        stats: *stats,
        name,
        species,
        class,
        level,
        experience,
    })
}

/// Generate a full game state snapshot for a newly connected player
pub fn generate_snapshot<'a>(
    builder: &'a mut FlatBufferBuilder<'a>,
    game_state: &GameState,
    _client_id: u64,
    player_entity: Entity,
) -> WIPOffset<GameStateSnapshot<'a>> {
    // Get player position to determine what entities are in vision range
    let player_pos = if let Ok(pos) = game_state.world.get::<&Position>(player_entity) {
        (pos.x, pos.y)
    } else {
        (1600.0, 1600.0) // Default center position
    };

    // Collect all entities within vision range
    let mut entity_infos = Vec::new();
    for (entity, position) in game_state.world.query::<&Position>().iter() {
        let distance = ((position.x - player_pos.0).powi(2) + (position.y - player_pos.1).powi(2)).sqrt();
        if distance <= VISION_RANGE_PIXELS {
            if let Some(info) = collect_entity_info(&game_state.world, entity) {
                entity_infos.push(info);
            }
        }
    }

    // PHASE 1: Create all string offsets first
    let map_id = builder.create_string("overworld_01");
    let map_name = builder.create_string("Overworld - Starting Area");
    let background_music = builder.create_string("overworld_theme");
    let ambient_sound = builder.create_string("forest_birds");

    // Create string offsets for all entities
    let mut entity_string_offsets = Vec::new();
    for info in &entity_infos {
        let sprite_id = builder.create_string(&info.sprite_id);
        let name = builder.create_string(&info.name);
        entity_string_offsets.push((sprite_id, name));
    }

    // PHASE 2: Build all EntityData objects using pre-created string offsets
    let mut entity_data_offsets = Vec::new();
    for (info, (sprite_id, name)) in entity_infos.iter().zip(entity_string_offsets.iter()) {
        // Build Vec2 tables (Vec2 is now a table, not a struct)
        let pos = Vec2::create(builder, &Vec2Args {
            x: info.pos.0,
            y: info.pos.1,
        });
        let vel = Vec2::create(builder, &Vec2Args {
            x: info.vel.0,
            y: info.vel.1,
        });

        // Build Stats struct
        let fb_stats = FbStats::new(
            info.stats.str,
            info.stats.dex,
            info.stats.con,
            info.stats.int,
            info.stats.wis,
            info.stats.cha,
        );

        // Convert species and class
        let fb_species = match info.species {
            Species::Human => network::Species::Human,
            Species::Elf => network::Species::Elf,
            Species::Dwarf => network::Species::Dwarf,
            Species::Halfling => network::Species::Halfling,
            Species::HalfOrc => network::Species::HalfOrc,
            Species::Gnome => network::Species::Gnome,
        };

        let fb_class = match info.class {
            CharacterClass::Fighter => network::CharacterClass::Fighter,
            CharacterClass::Rogue => network::CharacterClass::Rogue,
            CharacterClass::Cleric => network::CharacterClass::Cleric,
            CharacterClass::Wizard => network::CharacterClass::Wizard,
            CharacterClass::Ranger => network::CharacterClass::Ranger,
            CharacterClass::Barbarian => network::CharacterClass::Barbarian,
        };

        let entity_data_args = EntityDataArgs {
            id: info.id,
            position: Some(pos),
            velocity: Some(vel),
            sprite_id: Some(*sprite_id),
            health_current: info.health_current,
            health_max: info.health_max,
            stats: Some(&fb_stats),
            name: Some(*name),
            species: fb_species,
            character_class: fb_class,
            level: info.level,
            experience: info.experience,
        };

        entity_data_offsets.push(EntityData::create(builder, &entity_data_args));
    }

    // PHASE 3: Create vectors and final snapshot
    let entities_vector = builder.create_vector(&entity_data_offsets);
    let chunks_vector = builder.create_vector::<WIPOffset<network::ChunkData>>(&[]);

    let player_entity_id = player_entity.to_bits().get() as u32;

    let snapshot_args = GameStateSnapshotArgs {
        map_id: Some(map_id),
        map_name: Some(map_name),
        background_music: Some(background_music),
        ambient_sound: Some(ambient_sound),
        player_entity_id,
        entities: Some(entities_vector),
        chunks: Some(chunks_vector),
    };

    GameStateSnapshot::create(builder, &snapshot_args)
}

/// Generate a delta update containing only changed entities
pub fn generate_delta<'a>(
    builder: &'a mut FlatBufferBuilder<'a>,
    game_state: &GameState,
    changes: &EntityChanges,
    sequence: u32,
) -> WIPOffset<GameStateDelta<'a>> {
    // Collect updated entity infos
    let updated_infos: Vec<_> = changes
        .updated
        .iter()
        .filter_map(|&entity| collect_entity_info(&game_state.world, entity))
        .collect();

    // Collect spawned entity infos
    let spawned_infos: Vec<_> = changes
        .spawned
        .iter()
        .filter_map(|&entity| collect_entity_info(&game_state.world, entity))
        .collect();

    // PHASE 1: Create all string offsets first
    let mut updated_string_offsets = Vec::new();
    for info in &updated_infos {
        let sprite_id = builder.create_string(&info.sprite_id);
        let name = builder.create_string(&info.name);
        updated_string_offsets.push((sprite_id, name));
    }

    let mut spawned_string_offsets = Vec::new();
    for info in &spawned_infos {
        let sprite_id = builder.create_string(&info.sprite_id);
        let name = builder.create_string(&info.name);
        spawned_string_offsets.push((sprite_id, name));
    }

    // PHASE 2: Build EntityData objects
    let mut updated_data_offsets = Vec::new();
    for (info, (sprite_id, name)) in updated_infos.iter().zip(updated_string_offsets.iter()) {
        let pos = Vec2::create(builder, &Vec2Args {
            x: info.pos.0,
            y: info.pos.1,
        });
        let vel = Vec2::create(builder, &Vec2Args {
            x: info.vel.0,
            y: info.vel.1,
        });
        let fb_stats = FbStats::new(
            info.stats.str,
            info.stats.dex,
            info.stats.con,
            info.stats.int,
            info.stats.wis,
            info.stats.cha,
        );

        let fb_species = match info.species {
            Species::Human => network::Species::Human,
            Species::Elf => network::Species::Elf,
            Species::Dwarf => network::Species::Dwarf,
            Species::Halfling => network::Species::Halfling,
            Species::HalfOrc => network::Species::HalfOrc,
            Species::Gnome => network::Species::Gnome,
        };

        let fb_class = match info.class {
            CharacterClass::Fighter => network::CharacterClass::Fighter,
            CharacterClass::Rogue => network::CharacterClass::Rogue,
            CharacterClass::Cleric => network::CharacterClass::Cleric,
            CharacterClass::Wizard => network::CharacterClass::Wizard,
            CharacterClass::Ranger => network::CharacterClass::Ranger,
            CharacterClass::Barbarian => network::CharacterClass::Barbarian,
        };

        let entity_data_args = EntityDataArgs {
            id: info.id,
            position: Some(pos),
            velocity: Some(vel),
            sprite_id: Some(*sprite_id),
            health_current: info.health_current,
            health_max: info.health_max,
            stats: Some(&fb_stats),
            name: Some(*name),
            species: fb_species,
            character_class: fb_class,
            level: info.level,
            experience: info.experience,
        };

        updated_data_offsets.push(EntityData::create(builder, &entity_data_args));
    }

    let mut spawned_data_offsets = Vec::new();
    for (info, (sprite_id, name)) in spawned_infos.iter().zip(spawned_string_offsets.iter()) {
        let pos = Vec2::create(builder, &Vec2Args {
            x: info.pos.0,
            y: info.pos.1,
        });
        let vel = Vec2::create(builder, &Vec2Args {
            x: info.vel.0,
            y: info.vel.1,
        });
        let fb_stats = FbStats::new(
            info.stats.str,
            info.stats.dex,
            info.stats.con,
            info.stats.int,
            info.stats.wis,
            info.stats.cha,
        );

        let fb_species = match info.species {
            Species::Human => network::Species::Human,
            Species::Elf => network::Species::Elf,
            Species::Dwarf => network::Species::Dwarf,
            Species::Halfling => network::Species::Halfling,
            Species::HalfOrc => network::Species::HalfOrc,
            Species::Gnome => network::Species::Gnome,
        };

        let fb_class = match info.class {
            CharacterClass::Fighter => network::CharacterClass::Fighter,
            CharacterClass::Rogue => network::CharacterClass::Rogue,
            CharacterClass::Cleric => network::CharacterClass::Cleric,
            CharacterClass::Wizard => network::CharacterClass::Wizard,
            CharacterClass::Ranger => network::CharacterClass::Ranger,
            CharacterClass::Barbarian => network::CharacterClass::Barbarian,
        };

        let entity_data_args = EntityDataArgs {
            id: info.id,
            position: Some(pos),
            velocity: Some(vel),
            sprite_id: Some(*sprite_id),
            health_current: info.health_current,
            health_max: info.health_max,
            stats: Some(&fb_stats),
            name: Some(*name),
            species: fb_species,
            character_class: fb_class,
            level: info.level,
            experience: info.experience,
        };

        spawned_data_offsets.push(EntityData::create(builder, &entity_data_args));
    }

    // PHASE 3: Create vectors and final delta
    let updated_vector = builder.create_vector(&updated_data_offsets);
    let spawned_vector = builder.create_vector(&spawned_data_offsets);
    let despawned_vector = builder.create_vector(&changes.despawned);

    let delta_args = GameStateDeltaArgs {
        sequence,
        entities_updated: Some(updated_vector),
        entities_spawned: Some(spawned_vector),
        entities_despawned: Some(despawned_vector),
    };

    GameStateDelta::create(builder, &delta_args)
}

/// Build a complete snapshot message and return the bytes
fn build_snapshot_message(
    game_state: &GameState,
    _client_id: u64,
    player_entity: Entity,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    // Inline snapshot generation to avoid borrow checker issues
    let player_pos = if let Ok(pos) = game_state.world.get::<&Position>(player_entity) {
        (pos.x, pos.y)
    } else {
        (1600.0, 1600.0)
    };

    let mut entity_infos = Vec::new();
    for (entity, position) in game_state.world.query::<&Position>().iter() {
        let distance = ((position.x - player_pos.0).powi(2) + (position.y - player_pos.1).powi(2)).sqrt();
        if distance <= VISION_RANGE_PIXELS {
            if let Some(info) = collect_entity_info(&game_state.world, entity) {
                entity_infos.push(info);
            }
        }
    }

    let map_id = builder.create_string("overworld_01");
    let map_name = builder.create_string("Overworld - Starting Area");
    let background_music = builder.create_string("overworld_theme");
    let ambient_sound = builder.create_string("forest_birds");

    let mut entity_string_offsets = Vec::new();
    for info in &entity_infos {
        let sprite_id = builder.create_string(&info.sprite_id);
        let name = builder.create_string(&info.name);
        entity_string_offsets.push((sprite_id, name));
    }

    let mut entity_data_offsets = Vec::new();
    for (info, (sprite_id, name)) in entity_infos.iter().zip(entity_string_offsets.iter()) {
        let pos = Vec2::create(&mut builder, &Vec2Args {
            x: info.pos.0,
            y: info.pos.1,
        });
        let vel = Vec2::create(&mut builder, &Vec2Args {
            x: info.vel.0,
            y: info.vel.1,
        });
        let fb_stats = FbStats::new(info.stats.str, info.stats.dex, info.stats.con, info.stats.int, info.stats.wis, info.stats.cha);

        let fb_species = match info.species {
            Species::Human => network::Species::Human,
            Species::Elf => network::Species::Elf,
            Species::Dwarf => network::Species::Dwarf,
            Species::Halfling => network::Species::Halfling,
            Species::HalfOrc => network::Species::HalfOrc,
            Species::Gnome => network::Species::Gnome,
        };

        let fb_class = match info.class {
            CharacterClass::Fighter => network::CharacterClass::Fighter,
            CharacterClass::Rogue => network::CharacterClass::Rogue,
            CharacterClass::Cleric => network::CharacterClass::Cleric,
            CharacterClass::Wizard => network::CharacterClass::Wizard,
            CharacterClass::Ranger => network::CharacterClass::Ranger,
            CharacterClass::Barbarian => network::CharacterClass::Barbarian,
        };

        entity_data_offsets.push(EntityData::create(
            &mut builder,
            &EntityDataArgs {
                id: info.id,
                position: Some(pos),
                velocity: Some(vel),
                sprite_id: Some(*sprite_id),
                health_current: info.health_current,
                health_max: info.health_max,
                stats: Some(&fb_stats),
                name: Some(*name),
                species: fb_species,
                character_class: fb_class,
                level: info.level,
                experience: info.experience,
            },
        ));
    }

    let entities_vector = builder.create_vector(&entity_data_offsets);
    let chunks_vector = builder.create_vector::<WIPOffset<network::ChunkData>>(&[]);
    let player_entity_id = player_entity.to_bits().get() as u32;

    let snapshot = GameStateSnapshot::create(
        &mut builder,
        &GameStateSnapshotArgs {
            map_id: Some(map_id),
            map_name: Some(map_name),
            background_music: Some(background_music),
            ambient_sound: Some(ambient_sound),
            player_entity_id,
            entities: Some(entities_vector),
            chunks: Some(chunks_vector),
        },
    );

    let message = Message::create(
        &mut builder,
        &MessageArgs {
            payload_type: MessageType::GameStateSnapshot,
            payload: Some(snapshot.as_union_value()),
        },
    );

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

/// Send a GameStateSnapshot to a specific client
pub async fn send_snapshot_to_client(
    client: &ClientConnection,
    game_state: &GameState,
    player_entity: Entity,
) {
    let data = build_snapshot_message(game_state, client.id, player_entity);
    let data_len = data.len();

    if let Err(e) = client.send_message(data) {
        warn!("Failed to send snapshot to client {}: {}", client.id, e);
    } else {
        debug!("📤 Sent snapshot to client {} ({} bytes)", client.id, data_len);
    }
}

/// Build a complete delta message and return the bytes
fn build_delta_message(
    game_state: &GameState,
    changes: &EntityChanges,
    sequence: u32,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    // Inline delta generation to avoid borrow checker issues
    let updated_infos: Vec<_> = changes
        .updated
        .iter()
        .filter_map(|&entity| collect_entity_info(&game_state.world, entity))
        .collect();

    let spawned_infos: Vec<_> = changes
        .spawned
        .iter()
        .filter_map(|&entity| collect_entity_info(&game_state.world, entity))
        .collect();

    let mut updated_string_offsets = Vec::new();
    for info in &updated_infos {
        let sprite_id = builder.create_string(&info.sprite_id);
        let name = builder.create_string(&info.name);
        updated_string_offsets.push((sprite_id, name));
    }

    let mut spawned_string_offsets = Vec::new();
    for info in &spawned_infos {
        let sprite_id = builder.create_string(&info.sprite_id);
        let name = builder.create_string(&info.name);
        spawned_string_offsets.push((sprite_id, name));
    }

    let mut updated_data_offsets = Vec::new();
    for (info, (sprite_id, name)) in updated_infos.iter().zip(updated_string_offsets.iter()) {
        let pos = Vec2::create(&mut builder, &Vec2Args {
            x: info.pos.0,
            y: info.pos.1,
        });
        let vel = Vec2::create(&mut builder, &Vec2Args {
            x: info.vel.0,
            y: info.vel.1,
        });
        let fb_stats = FbStats::new(info.stats.str, info.stats.dex, info.stats.con, info.stats.int, info.stats.wis, info.stats.cha);

        let fb_species = match info.species {
            Species::Human => network::Species::Human,
            Species::Elf => network::Species::Elf,
            Species::Dwarf => network::Species::Dwarf,
            Species::Halfling => network::Species::Halfling,
            Species::HalfOrc => network::Species::HalfOrc,
            Species::Gnome => network::Species::Gnome,
        };

        let fb_class = match info.class {
            CharacterClass::Fighter => network::CharacterClass::Fighter,
            CharacterClass::Rogue => network::CharacterClass::Rogue,
            CharacterClass::Cleric => network::CharacterClass::Cleric,
            CharacterClass::Wizard => network::CharacterClass::Wizard,
            CharacterClass::Ranger => network::CharacterClass::Ranger,
            CharacterClass::Barbarian => network::CharacterClass::Barbarian,
        };

        updated_data_offsets.push(EntityData::create(
            &mut builder,
            &EntityDataArgs {
                id: info.id,
                position: Some(pos),
                velocity: Some(vel),
                sprite_id: Some(*sprite_id),
                health_current: info.health_current,
                health_max: info.health_max,
                stats: Some(&fb_stats),
                name: Some(*name),
                species: fb_species,
                character_class: fb_class,
                level: info.level,
                experience: info.experience,
            },
        ));
    }

    let mut spawned_data_offsets = Vec::new();
    for (info, (sprite_id, name)) in spawned_infos.iter().zip(spawned_string_offsets.iter()) {
        let pos = Vec2::create(&mut builder, &Vec2Args {
            x: info.pos.0,
            y: info.pos.1,
        });
        let vel = Vec2::create(&mut builder, &Vec2Args {
            x: info.vel.0,
            y: info.vel.1,
        });
        let fb_stats = FbStats::new(info.stats.str, info.stats.dex, info.stats.con, info.stats.int, info.stats.wis, info.stats.cha);

        let fb_species = match info.species {
            Species::Human => network::Species::Human,
            Species::Elf => network::Species::Elf,
            Species::Dwarf => network::Species::Dwarf,
            Species::Halfling => network::Species::Halfling,
            Species::HalfOrc => network::Species::HalfOrc,
            Species::Gnome => network::Species::Gnome,
        };

        let fb_class = match info.class {
            CharacterClass::Fighter => network::CharacterClass::Fighter,
            CharacterClass::Rogue => network::CharacterClass::Rogue,
            CharacterClass::Cleric => network::CharacterClass::Cleric,
            CharacterClass::Wizard => network::CharacterClass::Wizard,
            CharacterClass::Ranger => network::CharacterClass::Ranger,
            CharacterClass::Barbarian => network::CharacterClass::Barbarian,
        };

        spawned_data_offsets.push(EntityData::create(
            &mut builder,
            &EntityDataArgs {
                id: info.id,
                position: Some(pos),
                velocity: Some(vel),
                sprite_id: Some(*sprite_id),
                health_current: info.health_current,
                health_max: info.health_max,
                stats: Some(&fb_stats),
                name: Some(*name),
                species: fb_species,
                character_class: fb_class,
                level: info.level,
                experience: info.experience,
            },
        ));
    }

    let updated_vector = builder.create_vector(&updated_data_offsets);
    let spawned_vector = builder.create_vector(&spawned_data_offsets);
    let despawned_vector = builder.create_vector(&changes.despawned);

    let delta = GameStateDelta::create(
        &mut builder,
        &GameStateDeltaArgs {
            sequence,
            entities_updated: Some(updated_vector),
            entities_spawned: Some(spawned_vector),
            entities_despawned: Some(despawned_vector),
        },
    );

    let message = Message::create(
        &mut builder,
        &MessageArgs {
            payload_type: MessageType::GameStateDelta,
            payload: Some(delta.as_union_value()),
        },
    );

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

/// Broadcast a delta update to all connected clients (filtered by vision range)
pub async fn broadcast_delta(
    clients: &HashMap<u64, ClientConnection>,
    game_state: &GameState,
    changes: &EntityChanges,
    sequence: u32,
) {
    // For each client, filter entities by their vision range and send appropriate delta
    for (client_id, client) in clients.iter() {
        // Get player entity position
        let player_entity = match game_state.get_player_entity(*client_id) {
            Some(entity) => entity,
            None => continue,
        };

        let player_pos = match game_state.world.get::<&Position>(player_entity) {
            Ok(pos) => (pos.x, pos.y),
            Err(_) => continue,
        };

        // Filter entities by vision range
        let filtered_changes = filter_changes_by_vision(&game_state.world, changes, player_pos);

        // Only send if there are changes
        if filtered_changes.spawned.is_empty()
            && filtered_changes.updated.is_empty()
            && filtered_changes.despawned.is_empty()
        {
            continue;
        }

        // Build and send delta
        let data = build_delta_message(game_state, &filtered_changes, sequence);
        let data_len = data.len();

        if let Err(e) = client.send_message(data) {
            warn!("Failed to send delta to client {}: {}", client_id, e);
        } else {
            debug!(
                "📤 Sent delta to client {} ({} bytes): spawned={}, updated={}, despawned={}",
                client_id,
                data_len,
                filtered_changes.spawned.len(),
                filtered_changes.updated.len(),
                filtered_changes.despawned.len()
            );
        }
    }
}

/// Filter entity changes to only include entities within vision range
fn filter_changes_by_vision(
    world: &hecs::World,
    changes: &EntityChanges,
    player_pos: (f32, f32),
) -> EntityChanges {
    let is_in_range = |entity: Entity| -> bool {
        if let Ok(pos) = world.get::<&Position>(entity) {
            let distance =
                ((pos.x - player_pos.0).powi(2) + (pos.y - player_pos.1).powi(2)).sqrt();
            distance <= VISION_RANGE_PIXELS
        } else {
            false
        }
    };

    EntityChanges {
        spawned: changes
            .spawned
            .iter()
            .filter(|&&e| is_in_range(e))
            .copied()
            .collect(),
        updated: changes.updated.iter().filter(|&&e| is_in_range(e)).copied().collect(),
        despawned: changes.despawned.clone(),
    }
}
