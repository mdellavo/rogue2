use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use hecs::{World, Entity};

use crate::ecs::components::*;

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
}

pub struct GameState {
    pub world: World,
    pub players: HashMap<u64, PlayerState>, // connection_id -> PlayerState
    pub tick_count: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            players: HashMap::new(),
            tick_count: 0,
        }
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
        });

        entity
    }

    pub fn remove_player(&mut self, connection_id: u64) {
        if let Some(player_state) = self.players.remove(&connection_id) {
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
}

pub type SharedGameState = Arc<RwLock<GameState>>;
