use game_server::config::Config;
use game_server::game::state::GameState;
use game_server::game::r#loop::GameLoop;
use game_server::network;

use log::{info, error};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    // Load configuration
    let config = Config::from_env();
    info!("🚀 Starting game server...");
    info!("   Bind address: {}", config.bind_address());
    info!("   Max players: {}", config.max_players);
    info!("   Tick rate: {} Hz", config.tick_rate);

    // Initialize shared game state
    let game_state = Arc::new(RwLock::new(GameState::new()));
    info!("✅ Game state initialized");

    // Initialize WebSocket server
    let server = network::server::GameServer::new(Arc::clone(&game_state));
    let clients = server.get_clients();

    // Initialize game loop
    let game_loop = GameLoop::new(Arc::clone(&game_state), clients);

    info!("✅ Server ready!");

    // Start both WebSocket server and game loop
    tokio::select! {
        result = server.start(config.bind_address()) => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = game_loop.run() => {
            error!("Game loop unexpectedly stopped");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("🛑 Server shutting down...");
        }
    }

    Ok(())
}
