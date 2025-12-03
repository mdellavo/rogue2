// Procedural map generation orchestrator

use log::info;
use crate::map::types::{MapData, SpawnPoint, MapObject};
use crate::map::noise::NoiseGenerator;
use crate::map::biome::{BiomeType, BiomeRules};
use crate::map::features::FeatureGenerator;

pub struct MapGenerator {
    noise: NoiseGenerator,
    features: FeatureGenerator,
}

#[derive(Clone)]
pub struct GenerationConfig {
    pub seed: u32,
    pub width: u32,
    pub height: u32,
    pub map_id: String,
    pub map_name: String,
}

impl MapGenerator {
    pub fn new(seed: u32) -> Self {
        info!("🌱 Initializing map generator: seed={}", seed);

        Self {
            noise: NoiseGenerator::new(seed),
            features: FeatureGenerator::new(seed as u64),
        }
    }

    pub fn generate(&mut self, config: GenerationConfig) -> MapData {
        info!("🗺️  Generating map '{}' ({}x{})...",
              config.map_name, config.width, config.height);

        let tile_count = (config.width * config.height) as usize;
        let mut tile_data = Vec::with_capacity(tile_count);
        let mut objects = Vec::new();
        let mut spawn_points = Vec::new();

        // Track biome counts for music selection
        let mut biome_counts = std::collections::HashMap::new();

        // Generate terrain
        for y in 0..config.height {
            for x in 0..config.width {
                let world_x = x as f64;
                let world_y = y as f64;

                // Sample noise functions
                let elevation = self.noise.get_elevation(world_x, world_y);
                let moisture = self.noise.get_moisture(world_x, world_y);
                let temperature = self.noise.get_temperature(world_x, world_y, elevation);
                let detail = self.noise.get_detail(world_x, world_y);

                // Determine biome
                let rules = BiomeRules {
                    elevation,
                    moisture,
                    temperature,
                };
                let biome = rules.determine_biome();

                // Track biome for statistics
                *biome_counts.entry(biome).or_insert(0) += 1;

                // Select tile sprite ID
                let tile_id = self.get_tile_for_biome(biome, detail);
                tile_data.push(tile_id);

                // Place features (trees, rocks, etc.)
                if let Some(feature_type) = self.features.should_place_feature(biome, detail) {
                    objects.push(MapObject {
                        id: format!("{}_{}_{}_{}", feature_type, x, y, config.seed),
                        x: (x * 32) as f32,  // Convert to pixel coordinates
                        y: (y * 32) as f32,
                        object_type: feature_type.to_string(),
                    });
                }

                // Identify spawn points (flat, safe grassland areas)
                if biome == BiomeType::Grassland && elevation > 0.1 && elevation < 0.25 {
                    // Space them out evenly
                    if spawn_points.len() < 10 && x % 20 == 0 && y % 20 == 0 {
                        spawn_points.push(SpawnPoint {
                            x: (x * 32) as f32,
                            y: (y * 32) as f32,
                        });
                    }
                }
            }
        }

        // Ensure at least one spawn point in the center
        if spawn_points.is_empty() {
            info!("⚠️  No valid spawn points found, using center of map");
            spawn_points.push(SpawnPoint {
                x: (config.width * 16) as f32,
                y: (config.height * 16) as f32,
            });
        }

        // Log biome distribution
        info!("📊 Biome distribution:");
        for (biome, count) in biome_counts.iter() {
            let percentage = (*count as f32 / tile_count as f32) * 100.0;
            info!("   {} - {:.1}%", biome.name(), percentage);
        }

        info!("✅ Generated map: {} tiles, {} objects, {} spawn points",
              tile_data.len(), objects.len(), spawn_points.len());

        MapData {
            id: config.map_id,
            name: config.map_name,
            width: config.width,
            height: config.height,
            background_music: self.select_music_for_biomes(&biome_counts, tile_count),
            ambient_sound: "wind_ambient".to_string(),
            tile_data,
            spawn_points,
            objects,
        }
    }

    /// Get tile sprite ID for biome with variation
    ///
    /// Note: These are placeholder IDs. You'll need to map these to actual sprite IDs
    /// from your tileset once sprites.json is available.
    fn get_tile_for_biome(&self, biome: BiomeType, detail: f32) -> u32 {
        match biome {
            BiomeType::DeepWater => 100,
            BiomeType::ShallowWater => 101,
            BiomeType::Beach => 102,
            BiomeType::Grassland => {
                // Add variation: 80% grass_01, 10% grass_02, 10% grass_03
                if detail < 0.8 {
                    0
                } else if detail < 0.9 {
                    1
                } else {
                    2
                }
            }
            BiomeType::Forest => {
                // Forest floor variations
                if detail < 0.7 {
                    10
                } else {
                    11
                }
            }
            BiomeType::Desert => {
                // Desert sand variations
                if detail < 0.6 {
                    20
                } else if detail < 0.85 {
                    21
                } else {
                    22
                }
            }
            BiomeType::Hills => {
                // Hill grass/dirt
                if detail < 0.5 {
                    30
                } else {
                    31
                }
            }
            BiomeType::Mountains => {
                // Mountain rock
                if detail < 0.6 {
                    40
                } else {
                    41
                }
            }
            BiomeType::SnowPeaks => {
                // Snow
                if detail < 0.7 {
                    50
                } else {
                    51
                }
            }
        }
    }

    /// Select background music based on dominant terrain
    fn select_music_for_biomes(
        &self,
        biome_counts: &std::collections::HashMap<BiomeType, usize>,
        total_tiles: usize,
    ) -> String {
        let total = total_tiles as f32;

        // Calculate percentages
        let water_pct = (*biome_counts.get(&BiomeType::DeepWater).unwrap_or(&0)
            + *biome_counts.get(&BiomeType::ShallowWater).unwrap_or(&0)) as f32 / total;
        let desert_pct = *biome_counts.get(&BiomeType::Desert).unwrap_or(&0) as f32 / total;
        let mountain_pct = (*biome_counts.get(&BiomeType::Mountains).unwrap_or(&0)
            + *biome_counts.get(&BiomeType::SnowPeaks).unwrap_or(&0)) as f32 / total;
        let forest_pct = *biome_counts.get(&BiomeType::Forest).unwrap_or(&0) as f32 / total;

        // Select music based on dominant biome
        if water_pct > 0.3 {
            "ocean_theme".to_string()
        } else if desert_pct > 0.4 {
            "desert_theme".to_string()
        } else if mountain_pct > 0.3 {
            "mountain_theme".to_string()
        } else if forest_pct > 0.4 {
            "forest_theme".to_string()
        } else {
            "overworld_theme".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_generation() {
        let mut generator = MapGenerator::new(12345);

        let config = GenerationConfig {
            seed: 12345,
            width: 64,
            height: 64,
            map_id: "test_map".to_string(),
            map_name: "Test Map".to_string(),
        };

        let map = generator.generate(config.clone());

        // Verify basic properties
        assert_eq!(map.id, "test_map");
        assert_eq!(map.name, "Test Map");
        assert_eq!(map.width, 64);
        assert_eq!(map.height, 64);
        assert_eq!(map.tile_data.len(), 64 * 64);
        assert!(!map.spawn_points.is_empty(), "Should have at least one spawn point");
    }

    #[test]
    fn test_deterministic_generation() {
        let mut gen1 = MapGenerator::new(42);
        let mut gen2 = MapGenerator::new(42);

        let config = GenerationConfig {
            seed: 42,
            width: 32,
            height: 32,
            map_id: "test".to_string(),
            map_name: "Test".to_string(),
        };

        let map1 = gen1.generate(config.clone());
        let map2 = gen2.generate(config);

        // Same seed should produce identical maps
        assert_eq!(map1.tile_data, map2.tile_data);
        assert_eq!(map1.objects.len(), map2.objects.len());
    }
}
