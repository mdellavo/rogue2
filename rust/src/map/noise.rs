// Multi-octave noise generation for terrain

use noise::{NoiseFn, Perlin};

pub struct NoiseGenerator {
    #[allow(dead_code)]
    seed: u32,
    elevation: Perlin,
    moisture: Perlin,
    temperature: Perlin,
    detail: Perlin,  // Fine details for variation
}

impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            elevation: Perlin::new(seed),
            moisture: Perlin::new(seed.wrapping_add(1)),
            temperature: Perlin::new(seed.wrapping_add(2)),
            detail: Perlin::new(seed.wrapping_add(3)),
        }
    }

    /// Get elevation at world position (-1.0 to 1.0)
    pub fn get_elevation(&self, x: f64, y: f64) -> f32 {
        // Multi-octave noise for natural terrain
        let scale1 = 0.005;  // Large features (continents)
        let scale2 = 0.02;   // Medium features (hills)
        let scale3 = 0.08;   // Small features (bumps)

        let n1 = self.elevation.get([x * scale1, y * scale1]) * 1.0;
        let n2 = self.elevation.get([x * scale2, y * scale2]) * 0.5;
        let n3 = self.elevation.get([x * scale3, y * scale3]) * 0.25;

        ((n1 + n2 + n3) / 1.75) as f32
    }

    /// Get moisture at world position (0.0 to 1.0)
    pub fn get_moisture(&self, x: f64, y: f64) -> f32 {
        let scale = 0.01;  // Large moisture regions
        let n = self.moisture.get([x * scale, y * scale]);
        ((n + 1.0) / 2.0) as f32  // Normalize to 0-1
    }

    /// Get temperature at world position (0.0 to 1.0)
    /// Based on latitude (y) and elevation
    pub fn get_temperature(&self, x: f64, y: f64, elevation: f32) -> f32 {
        let scale = 0.008;
        let base_temp = self.temperature.get([x * scale, y * scale]);

        // Latitude effect (colder at poles)
        let latitude_factor = (y * 0.0001).cos();

        // Elevation effect (colder at high altitude)
        let elevation_penalty = elevation.max(0.0) * 0.5;

        let temp = (base_temp as f32 + latitude_factor as f32 - elevation_penalty + 1.0) / 2.0;
        temp.clamp(0.0, 1.0)
    }

    /// Get fine detail variation (0.0 to 1.0)
    pub fn get_detail(&self, x: f64, y: f64) -> f32 {
        let scale = 0.2;  // High frequency for tile-level variation
        let n = self.detail.get([x * scale, y * scale]);
        ((n + 1.0) / 2.0) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_generation() {
        let generator = NoiseGenerator::new(12345);

        // Test elevation
        let elevation = generator.get_elevation(100.0, 100.0);
        assert!(elevation >= -1.0 && elevation <= 1.0);

        // Test moisture
        let moisture = generator.get_moisture(100.0, 100.0);
        assert!(moisture >= 0.0 && moisture <= 1.0);

        // Test temperature
        let temperature = generator.get_temperature(100.0, 100.0, 0.5);
        assert!(temperature >= 0.0 && temperature <= 1.0);

        // Test detail
        let detail = generator.get_detail(100.0, 100.0);
        assert!(detail >= 0.0 && detail <= 1.0);
    }

    #[test]
    fn test_deterministic() {
        let gen1 = NoiseGenerator::new(42);
        let gen2 = NoiseGenerator::new(42);

        // Same seed should produce same values
        let elev1 = gen1.get_elevation(50.0, 50.0);
        let elev2 = gen2.get_elevation(50.0, 50.0);
        assert_eq!(elev1, elev2);
    }
}
