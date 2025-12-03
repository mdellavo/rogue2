// Feature placement for procedural maps (trees, rocks, etc.)

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crate::map::biome::BiomeType;

pub struct FeatureGenerator {
    rng: ChaCha8Rng,
}

impl FeatureGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Determine if a feature should be placed at this location
    pub fn should_place_feature(
        &mut self,
        biome: BiomeType,
        detail_noise: f32,
    ) -> Option<&'static str> {
        let density = biome.feature_density();

        if density == 0.0 {
            return None;
        }

        // Combine noise and RNG for natural clustering
        let threshold = 1.0 - density;
        if detail_noise > threshold && self.rng.gen::<f32>() > 0.5 {
            return self.select_feature_for_biome(biome);
        }

        None
    }

    fn select_feature_for_biome(&mut self, biome: BiomeType) -> Option<&'static str> {
        match biome {
            BiomeType::Forest => {
                // 70% trees, 20% bushes, 10% rocks
                let roll = self.rng.gen::<f32>();
                if roll < 0.7 {
                    Some("tree_oak")
                } else if roll < 0.9 {
                    Some("bush_green")
                } else {
                    Some("rock_small")
                }
            }
            BiomeType::Grassland => {
                let roll = self.rng.gen::<f32>();
                if roll < 0.6 {
                    Some("tree_oak")
                } else if roll < 0.9 {
                    Some("bush_green")
                } else {
                    Some("rock_small")
                }
            }
            BiomeType::Desert => {
                let roll = self.rng.gen::<f32>();
                if roll < 0.5 {
                    Some("cactus")
                } else if roll < 0.8 {
                    Some("rock_desert")
                } else {
                    Some("dead_tree")
                }
            }
            BiomeType::Hills => {
                let roll = self.rng.gen::<f32>();
                if roll < 0.7 {
                    Some("rock_large")
                } else {
                    Some("bush_dry")
                }
            }
            BiomeType::Mountains => {
                Some("rock_boulder")
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_generation() {
        let mut generator = FeatureGenerator::new(12345);

        // Forest should have high chance of features
        let mut feature_count = 0;
        for _ in 0..100 {
            if generator.should_place_feature(BiomeType::Forest, 0.9).is_some() {
                feature_count += 1;
            }
        }
        assert!(feature_count > 0, "Forest should generate some features");

        // Water should never have features
        let mut generator2 = FeatureGenerator::new(12345);
        for _ in 0..100 {
            assert!(generator2.should_place_feature(BiomeType::DeepWater, 0.9).is_none());
        }
    }

    #[test]
    fn test_deterministic() {
        let mut gen1 = FeatureGenerator::new(42);
        let mut gen2 = FeatureGenerator::new(42);

        // Same seed should produce same features
        let feature1 = gen1.should_place_feature(BiomeType::Forest, 0.95);
        let feature2 = gen2.should_place_feature(BiomeType::Forest, 0.95);
        assert_eq!(feature1, feature2);
    }
}
