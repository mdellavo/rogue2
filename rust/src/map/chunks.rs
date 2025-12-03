// Chunk system for efficient map streaming

use std::collections::{HashMap, HashSet};
use crate::map::types::MapData;

/// Chunk system configuration
pub const CHUNK_SIZE: u32 = 32; // 32x32 tiles per chunk
pub const CHUNK_LOAD_RADIUS: i32 = 1; // Load 3x3 grid (1 chunk in each direction)

/// World position to chunk coordinate
pub fn world_to_chunk(world_x: f32, world_y: f32, tile_size: u32) -> (i32, i32) {
    let tile_x = (world_x / tile_size as f32).floor() as i32;
    let tile_y = (world_y / tile_size as f32).floor() as i32;
    (tile_x / CHUNK_SIZE as i32, tile_y / CHUNK_SIZE as i32)
}

/// Chunk coordinate to world position (top-left corner)
pub fn chunk_to_world(chunk_x: i32, chunk_y: i32, tile_size: u32) -> (f32, f32) {
    (
        (chunk_x * CHUNK_SIZE as i32) as f32 * tile_size as f32,
        (chunk_y * CHUNK_SIZE as i32) as f32 * tile_size as f32,
    )
}

/// Terrain type definition
#[derive(Debug, Clone)]
pub struct TerrainType {
    pub id: u32,
    pub terrain_type: String,
    pub walkable: bool,
    pub sprite_id: String,
}

/// Feature type definition
#[derive(Debug, Clone)]
pub struct FeatureType {
    pub id: u32,
    pub feature_type: String,
    pub blocks_movement: bool,
    pub sprite_id: String,
}

/// Feature within a chunk
#[derive(Debug, Clone)]
pub struct ChunkFeature {
    pub tile_x: u8,  // 0-31
    pub tile_y: u8,  // 0-31
    pub feature_id: u32,
}

/// A single chunk of terrain
#[derive(Debug, Clone)]
pub struct Chunk {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub tiles: Vec<u32>,  // 32*32 = 1024 tile IDs
    pub features: Vec<ChunkFeature>,
}

/// Complete chunk system for a map
pub struct ChunkSystem {
    pub map_width_chunks: u32,
    pub map_height_chunks: u32,
    pub terrain_index: Vec<TerrainType>,
    pub feature_index: Vec<FeatureType>,
    pub chunks: HashMap<(i32, i32), Chunk>,
}

impl ChunkSystem {
    /// Create chunk system from full map data
    pub fn from_map_data(map: &MapData) -> Self {
        let map_width_chunks = (map.width + CHUNK_SIZE - 1) / CHUNK_SIZE;
        let map_height_chunks = (map.height + CHUNK_SIZE - 1) / CHUNK_SIZE;

        // Build terrain index from tile data
        let terrain_index = Self::build_terrain_index();

        // Build feature index from objects
        let feature_index = Self::build_feature_index();

        // Divide map into chunks
        let chunks = Self::create_chunks(map, &feature_index);

        Self {
            map_width_chunks,
            map_height_chunks,
            terrain_index,
            feature_index,
            chunks,
        }
    }

    /// Build terrain index with all biome types
    fn build_terrain_index() -> Vec<TerrainType> {
        vec![
            TerrainType {
                id: 0,
                terrain_type: "grass_01".to_string(),
                walkable: true,
                sprite_id: "grass_01".to_string(),
            },
            TerrainType {
                id: 1,
                terrain_type: "grass_02".to_string(),
                walkable: true,
                sprite_id: "grass_02".to_string(),
            },
            TerrainType {
                id: 2,
                terrain_type: "grass_03".to_string(),
                walkable: true,
                sprite_id: "grass_03".to_string(),
            },
            TerrainType {
                id: 10,
                terrain_type: "forest_floor_01".to_string(),
                walkable: true,
                sprite_id: "forest_floor_01".to_string(),
            },
            TerrainType {
                id: 11,
                terrain_type: "forest_floor_02".to_string(),
                walkable: true,
                sprite_id: "forest_floor_02".to_string(),
            },
            TerrainType {
                id: 20,
                terrain_type: "desert_sand_01".to_string(),
                walkable: true,
                sprite_id: "desert_sand_01".to_string(),
            },
            TerrainType {
                id: 21,
                terrain_type: "desert_sand_02".to_string(),
                walkable: true,
                sprite_id: "desert_sand_02".to_string(),
            },
            TerrainType {
                id: 22,
                terrain_type: "desert_sand_03".to_string(),
                walkable: true,
                sprite_id: "desert_sand_03".to_string(),
            },
            TerrainType {
                id: 30,
                terrain_type: "hills_grass".to_string(),
                walkable: true,
                sprite_id: "hills_grass".to_string(),
            },
            TerrainType {
                id: 31,
                terrain_type: "hills_dirt".to_string(),
                walkable: true,
                sprite_id: "hills_dirt".to_string(),
            },
            TerrainType {
                id: 40,
                terrain_type: "mountain_rock_01".to_string(),
                walkable: true,
                sprite_id: "mountain_rock_01".to_string(),
            },
            TerrainType {
                id: 41,
                terrain_type: "mountain_rock_02".to_string(),
                walkable: true,
                sprite_id: "mountain_rock_02".to_string(),
            },
            TerrainType {
                id: 50,
                terrain_type: "snow_01".to_string(),
                walkable: true,
                sprite_id: "snow_01".to_string(),
            },
            TerrainType {
                id: 51,
                terrain_type: "snow_02".to_string(),
                walkable: true,
                sprite_id: "snow_02".to_string(),
            },
            TerrainType {
                id: 100,
                terrain_type: "deep_water".to_string(),
                walkable: false,
                sprite_id: "water_deep".to_string(),
            },
            TerrainType {
                id: 101,
                terrain_type: "shallow_water".to_string(),
                walkable: false,
                sprite_id: "water_shallow".to_string(),
            },
            TerrainType {
                id: 102,
                terrain_type: "beach_sand".to_string(),
                walkable: true,
                sprite_id: "beach_sand".to_string(),
            },
        ]
    }

    /// Build feature index from map objects
    fn build_feature_index() -> Vec<FeatureType> {
        vec![
            FeatureType {
                id: 1,
                feature_type: "tree_oak".to_string(),
                blocks_movement: true,
                sprite_id: "tree_oak".to_string(),
            },
            FeatureType {
                id: 2,
                feature_type: "bush_green".to_string(),
                blocks_movement: false,
                sprite_id: "bush_green".to_string(),
            },
            FeatureType {
                id: 3,
                feature_type: "rock_small".to_string(),
                blocks_movement: true,
                sprite_id: "rock_small".to_string(),
            },
            FeatureType {
                id: 4,
                feature_type: "rock_large".to_string(),
                blocks_movement: true,
                sprite_id: "rock_large".to_string(),
            },
            FeatureType {
                id: 5,
                feature_type: "rock_boulder".to_string(),
                blocks_movement: true,
                sprite_id: "rock_boulder".to_string(),
            },
            FeatureType {
                id: 6,
                feature_type: "cactus".to_string(),
                blocks_movement: true,
                sprite_id: "cactus".to_string(),
            },
            FeatureType {
                id: 7,
                feature_type: "dead_tree".to_string(),
                blocks_movement: true,
                sprite_id: "dead_tree".to_string(),
            },
            FeatureType {
                id: 8,
                feature_type: "rock_desert".to_string(),
                blocks_movement: true,
                sprite_id: "rock_desert".to_string(),
            },
            FeatureType {
                id: 9,
                feature_type: "bush_dry".to_string(),
                blocks_movement: false,
                sprite_id: "bush_dry".to_string(),
            },
        ]
    }

    /// Create all chunks from map data
    fn create_chunks(
        map: &MapData,
        feature_index: &[FeatureType],
    ) -> HashMap<(i32, i32), Chunk> {
        let mut chunks = HashMap::new();

        let map_width_chunks = (map.width + CHUNK_SIZE - 1) / CHUNK_SIZE;
        let map_height_chunks = (map.height + CHUNK_SIZE - 1) / CHUNK_SIZE;

        for chunk_y in 0..map_height_chunks as i32 {
            for chunk_x in 0..map_width_chunks as i32 {
                let chunk = Self::create_chunk(map, chunk_x, chunk_y, feature_index);
                chunks.insert((chunk_x, chunk_y), chunk);
            }
        }

        chunks
    }

    /// Create a single chunk
    fn create_chunk(
        map: &MapData,
        chunk_x: i32,
        chunk_y: i32,
        feature_index: &[FeatureType],
    ) -> Chunk {
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);
        let mut features = Vec::new();

        // Extract tile data for this chunk
        for local_y in 0..CHUNK_SIZE {
            for local_x in 0..CHUNK_SIZE {
                let world_x = chunk_x as u32 * CHUNK_SIZE + local_x;
                let world_y = chunk_y as u32 * CHUNK_SIZE + local_y;

                if world_x < map.width && world_y < map.height {
                    let tile_index = (world_y * map.width + world_x) as usize;
                    tiles.push(map.tile_data[tile_index]);
                } else {
                    // Out of bounds - use default tile
                    tiles.push(0);
                }
            }
        }

        // Extract features in this chunk
        for obj in &map.objects {
            let obj_tile_x = (obj.x / 32.0) as i32;
            let obj_tile_y = (obj.y / 32.0) as i32;
            let obj_chunk_x = obj_tile_x / CHUNK_SIZE as i32;
            let obj_chunk_y = obj_tile_y / CHUNK_SIZE as i32;

            if obj_chunk_x == chunk_x && obj_chunk_y == chunk_y {
                // Feature is in this chunk
                let local_x = (obj_tile_x % CHUNK_SIZE as i32) as u8;
                let local_y = (obj_tile_y % CHUNK_SIZE as i32) as u8;

                // Find feature ID from index
                if let Some(feature_type) = feature_index.iter().find(|f| f.feature_type == obj.object_type) {
                    features.push(ChunkFeature {
                        tile_x: local_x,
                        tile_y: local_y,
                        feature_id: feature_type.id,
                    });
                }
            }
        }

        Chunk {
            chunk_x,
            chunk_y,
            tiles,
            features,
        }
    }

    /// Get chunks needed for a player position
    pub fn get_chunks_for_position(&self, world_x: f32, world_y: f32) -> Vec<(i32, i32)> {
        let (center_cx, center_cy) = world_to_chunk(world_x, world_y, 32);
        let mut chunk_coords = Vec::new();

        for dy in -CHUNK_LOAD_RADIUS..=CHUNK_LOAD_RADIUS {
            for dx in -CHUNK_LOAD_RADIUS..=CHUNK_LOAD_RADIUS {
                let cx = center_cx + dx;
                let cy = center_cy + dy;

                // Check bounds
                if cx >= 0 && cy >= 0
                    && cx < self.map_width_chunks as i32
                    && cy < self.map_height_chunks as i32
                {
                    chunk_coords.push((cx, cy));
                }
            }
        }

        chunk_coords
    }

    /// Get chunk by coordinates
    pub fn get_chunk(&self, chunk_x: i32, chunk_y: i32) -> Option<&Chunk> {
        self.chunks.get(&(chunk_x, chunk_y))
    }
}

/// Calculate chunk differences for a player
#[derive(Default, Debug)]
pub struct ChunkUpdate {
    pub to_load: Vec<(i32, i32)>,
    pub to_unload: Vec<(i32, i32)>,
}

pub fn calculate_chunk_update(
    current_chunks: &HashSet<(i32, i32)>,
    needed_chunks: &HashSet<(i32, i32)>,
) -> ChunkUpdate {
    let to_load: Vec<_> = needed_chunks.difference(current_chunks).copied().collect();
    let to_unload: Vec<_> = current_chunks.difference(needed_chunks).copied().collect();

    ChunkUpdate { to_load, to_unload }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_chunk() {
        // Tile size 32, chunk size 32
        assert_eq!(world_to_chunk(0.0, 0.0, 32), (0, 0));
        assert_eq!(world_to_chunk(1024.0, 1024.0, 32), (1, 1));
        assert_eq!(world_to_chunk(2048.0, 2048.0, 32), (2, 2));
        assert_eq!(world_to_chunk(500.0, 500.0, 32), (0, 0));
        assert_eq!(world_to_chunk(1100.0, 1100.0, 32), (1, 1));
    }

    #[test]
    fn test_chunk_to_world() {
        assert_eq!(chunk_to_world(0, 0, 32), (0.0, 0.0));
        assert_eq!(chunk_to_world(1, 1, 32), (1024.0, 1024.0));
        assert_eq!(chunk_to_world(2, 2, 32), (2048.0, 2048.0));
    }
}
