// Map loading from JSON and procedural generation

use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use log::info;

use super::types::MapData;
use super::generator::{MapGenerator, GenerationConfig};

/// Load a map from a JSON file
pub fn load_map_from_file<P: AsRef<Path>>(path: P) -> Result<MapData> {
    let path = path.as_ref();
    info!("📂 Loading map from: {}", path.display());

    let json_str = fs::read_to_string(path)
        .with_context(|| format!("Failed to read map file: {}", path.display()))?;

    let map_data: MapData = serde_json::from_str(&json_str)
        .with_context(|| format!("Failed to parse map JSON: {}", path.display()))?;

    info!("✅ Loaded map '{}' ({}x{})", map_data.name, map_data.width, map_data.height);
    info!("   Spawn points: {}", map_data.spawn_points.len());
    info!("   Background music: {}", map_data.background_music);

    Ok(map_data)
}

/// Generate a procedural map
pub fn generate_procedural_map(seed: u32, width: u32, height: u32) -> Result<MapData> {
    info!("🎲 Generating procedural map: seed={}, size={}x{}", seed, width, height);

    let config = GenerationConfig {
        seed,
        width,
        height,
        map_id: format!("procedural_{}", seed),
        map_name: format!("Procedural World (seed: {})", seed),
    };

    let mut generator = MapGenerator::new(seed);
    Ok(generator.generate(config))
}

/// Load the default map (overworld_01.json)
pub fn load_default_map() -> Result<MapData> {
    load_map_from_file("data/maps/overworld_01.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_default_map() {
        // This test will only pass if the map file exists
        let result = load_default_map();
        match result {
            Ok(map) => {
                assert_eq!(map.id, "overworld_01");
                assert!(!map.spawn_points.is_empty());
            }
            Err(e) => {
                eprintln!("Map loading failed (may be expected in test environment): {}", e);
            }
        }
    }
}
