use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub max_players: usize,
    pub tick_rate: u64,
    pub log_level: String,
    pub map: MapConfig,
}

#[derive(Debug, Clone)]
pub struct MapConfig {
    pub use_procedural: bool,
    pub procedural_seed: Option<u32>,
    pub procedural_width: u32,
    pub procedural_height: u32,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
            max_players: env::var("MAX_PLAYERS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .expect("MAX_PLAYERS must be a valid number"),
            tick_rate: 60, // 60 Hz
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            map: MapConfig::from_env(),
        }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl MapConfig {
    pub fn from_env() -> Self {
        Self {
            use_procedural: env::var("USE_PROCEDURAL_MAP")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            procedural_seed: env::var("PROCEDURAL_SEED")
                .ok()
                .and_then(|s| s.parse().ok()),
            procedural_width: env::var("PROCEDURAL_WIDTH")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(128),
            procedural_height: env::var("PROCEDURAL_HEIGHT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(128),
        }
    }
}
