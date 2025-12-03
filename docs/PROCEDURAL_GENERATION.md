# Procedural Map Generation

This document explains the procedural map generation system implemented using Perlin noise.

## Overview

The procedural generation system creates varied, natural-looking terrain with multiple biomes including:
- **Grassland** - Plains with occasional trees and bushes
- **Forest** - Dense vegetation with trees, bushes, and rocks
- **Desert** - Arid landscape with cacti, dead trees, and rocks
- **Hills** - Rolling terrain with rocks and sparse vegetation
- **Mountains** - Rocky high-altitude terrain
- **Snow Peaks** - Snowy mountain tops
- **Beach** - Sandy areas near water
- **Shallow Water** - Coastal waters
- **Deep Water** - Ocean

## Architecture

### Key Components

1. **NoiseGenerator** (`src/map/noise.rs`)
   - Multi-octave Perlin noise for elevation, moisture, temperature
   - Deterministic generation from seed

2. **BiomeRules** (`src/map/biome.rs`)
   - Determines biome based on elevation, moisture, and temperature
   - 9 distinct biome types with smooth transitions

3. **FeatureGenerator** (`src/map/features.rs`)
   - Places features (trees, rocks, cacti) based on biome
   - Uses noise-based clustering for natural distribution

4. **MapGenerator** (`src/map/generator.rs`)
   - Orchestrates terrain generation
   - Selects background music based on dominant biome
   - Identifies safe spawn points

## Usage

### Environment Variables

```bash
# Enable procedural generation (default: false)
USE_PROCEDURAL_MAP=true

# Set seed for reproducible generation (default: random timestamp)
PROCEDURAL_SEED=12345

# Set map dimensions in tiles (default: 128x128)
PROCEDURAL_WIDTH=256
PROCEDURAL_HEIGHT=256
```

### Examples

**Generate random procedural map:**
```bash
USE_PROCEDURAL_MAP=true cargo run
```

**Generate specific seed:**
```bash
USE_PROCEDURAL_MAP=true PROCEDURAL_SEED=42 cargo run
```

**Generate large map:**
```bash
USE_PROCEDURAL_MAP=true PROCEDURAL_WIDTH=512 PROCEDURAL_HEIGHT=512 cargo run
```

**Use static map (default):**
```bash
cargo run
```

## Technical Details

### Noise Functions

The system uses four separate Perlin noise functions:

1. **Elevation Noise** (3 octaves)
   - Large scale: Continental features (0.005)
   - Medium scale: Hills and valleys (0.02)
   - Small scale: Local bumps (0.08)

2. **Moisture Noise**
   - Single octave (0.01)
   - Determines dry vs wet regions

3. **Temperature Noise**
   - Affected by latitude and elevation
   - Colder at poles and high altitudes

4. **Detail Noise**
   - High frequency (0.2)
   - Used for tile variation and feature placement

### Biome Selection

Biomes are determined by:
1. **Primary:** Elevation thresholds
   - < -0.3: Deep Water
   - -0.3 to -0.1: Shallow Water
   - > 0.8: Snow Peaks
   - 0.6 to 0.8: Mountains
   - 0.3 to 0.6: Hills

2. **Secondary:** Moisture (for low elevation)
   - < 0.3: Desert
   - 0.3 to 0.6: Grassland
   - > 0.6: Forest

3. **Tertiary:** Proximity to water
   - Elevation < 0.05 + Moisture > 0.3: Beach

### Feature Density

Feature placement varies by biome:
- Forest: 15% density (trees, bushes, rocks)
- Mountains: 12% density (boulders)
- Hills: 8% density (rocks, bushes)
- Grassland: 5% density (trees, bushes, rocks)
- Desert: 3% density (cacti, rocks, dead trees)
- Water biomes: No features

### Music Selection

Background music is automatically selected based on dominant terrain:
- 30%+ water → "ocean_theme"
- 40%+ desert → "desert_theme"
- 30%+ mountains → "mountain_theme"
- 40%+ forest → "forest_theme"
- Default → "overworld_theme"

## Testing

### Unit Tests

Run the test suite:
```bash
cd rust && cargo test
```

Tests verify:
- Deterministic generation (same seed = same map)
- Noise value ranges (all within 0-1 or -1 to 1)
- Biome feature constraints (water = no features)
- Spawn point generation

### Integration Testing

Generate a test map:
```bash
USE_PROCEDURAL_MAP=true PROCEDURAL_SEED=12345 PROCEDURAL_WIDTH=64 PROCEDURAL_HEIGHT=64 RUST_LOG=info cargo run
```

Check the logs for:
- ✅ Map generation success
- 📊 Biome distribution percentages
- 🎵 Selected background music
- 🎯 Number of spawn points and objects

## Performance

Generation benchmarks (approximate):
- 64x64 tiles: < 10ms
- 128x128 tiles: ~20ms
- 256x256 tiles: ~80ms
- 512x512 tiles: ~300ms

The generation is fast enough to be done at server startup without noticeable delay.

## Future Enhancements

### Planned Features
1. **River Generation** - Use flow simulation for realistic rivers
2. **Road Networks** - Connect spawn points with paths
3. **Structure Placement** - Towns, dungeons, camps
4. **Biome Edge Blending** - Smooth transitions between biomes
5. **Cave Systems** - Underground areas with entrances
6. **Ore Deposits** - Resource nodes for mining

### Chunk-Based Generation (Phase 2+)
Currently generates entire map at startup. Future versions will support:
- On-demand chunk generation as players explore
- Infinite worlds with chunk unloading
- Per-chunk seed derivation for consistency

## Tileset Mapping

The generator currently uses placeholder tile IDs. To map to your tileset:

1. Identify sprite IDs in `web/public/manifests/sprites.json`
2. Update `get_tile_for_biome()` in `src/map/generator.rs`:

```rust
fn get_tile_for_biome(&self, biome: BiomeType, detail: f32) -> u32 {
    match biome {
        BiomeType::Grassland => {
            // Map to your grass sprite IDs
            if detail < 0.8 { YOUR_GRASS_01_ID }
            else if detail < 0.9 { YOUR_GRASS_02_ID }
            else { YOUR_GRASS_03_ID }
        }
        // ... map other biomes
    }
}
```

## Troubleshooting

**Issue:** Map generation is slow
- Reduce map size with `PROCEDURAL_WIDTH` and `PROCEDURAL_HEIGHT`
- Generation time scales with tile count (O(n²))

**Issue:** Not enough spawn points
- Lower elevation thresholds in biome determination
- Increase map size to have more grassland areas

**Issue:** Too many/few features
- Adjust `feature_density()` values in `src/map/biome.rs`
- Modify threshold in `should_place_feature()`

**Issue:** Unrealistic terrain
- Tune noise scales in `src/map/noise.rs`
- Adjust biome elevation thresholds in `src/map/biome.rs`

## References

- [Perlin Noise](https://en.wikipedia.org/wiki/Perlin_noise) - Coherent noise function
- [noise-rs](https://docs.rs/noise/) - Rust noise generation library
- [Procedural Generation Wiki](http://pcg.wikidot.com/) - PCG techniques
