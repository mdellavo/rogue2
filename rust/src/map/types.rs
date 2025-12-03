// Map data structures

use serde::{Deserialize, Serialize};

/// Represents a spawn point for players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPoint {
    pub x: f32,
    pub y: f32,
}

/// Represents a static map loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "backgroundMusic")]
    pub background_music: String,
    #[serde(rename = "ambientSound")]
    pub ambient_sound: String,
    #[serde(rename = "tileData")]
    pub tile_data: Vec<u32>,
    #[serde(rename = "spawnPoints")]
    pub spawn_points: Vec<SpawnPoint>,
    pub objects: Vec<MapObject>,
}

/// Represents an object placed in the map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapObject {
    pub id: String,
    pub x: f32,
    pub y: f32,
    #[serde(rename = "type")]
    pub object_type: String,
}
