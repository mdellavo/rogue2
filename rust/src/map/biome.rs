// Biome system for procedural map generation

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BiomeType {
    DeepWater,      // < -0.3 elevation
    ShallowWater,   // -0.3 to -0.1
    Beach,          // -0.1 to 0.0, near water
    Grassland,      // 0.0 to 0.3
    Forest,         // 0.0 to 0.3, high moisture
    Desert,         // 0.0 to 0.3, low moisture
    Hills,          // 0.3 to 0.6
    Mountains,      // 0.6 to 1.0
    SnowPeaks,      // > 0.8
}

pub struct BiomeConfig {
    pub biome_type: BiomeType,
    pub floor_tiles: Vec<&'static str>,  // Sprite IDs for floor
    pub wall_tiles: Vec<&'static str>,   // Sprite IDs for walls/cliffs
    pub feature_density: f32,            // 0.0 to 1.0
    pub walkable: bool,
}

pub struct BiomeRules {
    pub elevation: f32,      // -1.0 to 1.0
    pub moisture: f32,       // 0.0 to 1.0
    pub temperature: f32,    // 0.0 to 1.0 (derived from elevation)
}

impl BiomeRules {
    pub fn determine_biome(&self) -> BiomeType {
        // Elevation-based first
        if self.elevation < -0.3 {
            return BiomeType::DeepWater;
        }
        if self.elevation < -0.1 {
            return BiomeType::ShallowWater;
        }
        if self.elevation > 0.8 {
            return BiomeType::SnowPeaks;
        }
        if self.elevation > 0.6 {
            return BiomeType::Mountains;
        }
        if self.elevation > 0.3 {
            return BiomeType::Hills;
        }

        // Low elevation - moisture determines biome
        if self.elevation < 0.05 && self.moisture > 0.3 {
            return BiomeType::Beach; // Near water
        }
        if self.moisture < 0.3 {
            return BiomeType::Desert;
        }
        if self.moisture > 0.6 {
            return BiomeType::Forest;
        }

        BiomeType::Grassland
    }
}

impl BiomeType {
    /// Get a human-readable name for the biome
    pub fn name(&self) -> &'static str {
        match self {
            BiomeType::DeepWater => "Deep Water",
            BiomeType::ShallowWater => "Shallow Water",
            BiomeType::Beach => "Beach",
            BiomeType::Grassland => "Grassland",
            BiomeType::Forest => "Forest",
            BiomeType::Desert => "Desert",
            BiomeType::Hills => "Hills",
            BiomeType::Mountains => "Mountains",
            BiomeType::SnowPeaks => "Snow Peaks",
        }
    }

    /// Check if this biome is walkable
    pub fn is_walkable(&self) -> bool {
        match self {
            BiomeType::DeepWater | BiomeType::ShallowWater => false,
            _ => true,
        }
    }

    /// Get feature density for this biome (0.0 to 1.0)
    pub fn feature_density(&self) -> f32 {
        match self {
            BiomeType::Forest => 0.15,
            BiomeType::Grassland => 0.05,
            BiomeType::Desert => 0.03,
            BiomeType::Hills => 0.08,
            BiomeType::Mountains => 0.12,
            _ => 0.0,
        }
    }
}
