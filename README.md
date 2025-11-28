# Multiplayer Roguelike Game

A 2D sprite-based multiplayer roguelike game with TypeScript frontend and Rust backend.

## Project Structure

- `web/` - TypeScript frontend (Vite + WebGL)
- `rust/` - Rust backend (Tokio + WebSockets)
- `schema/` - FlatBuffers message schemas

## Quick Start

### Prerequisites
- Node.js 20.19+ or 22.12+ (required by Vite 7)
- Rust 1.75+
- FlatBuffers compiler (`brew install flatbuffers`)
- tmux (`brew install tmux`)

### Setup

1. **Generate FlatBuffers types:**
   ```bash
   cd schema
   ./generate.sh
   ```

2. **Install frontend dependencies:**
   ```bash
   cd web
   npm install
   ```

3. **Start both servers (recommended):**
   ```bash
   ./run.sh
   ```
   This starts both backend and frontend in a split tmux window.
   - Press `Ctrl+B` then `Up/Down arrow` to switch between panes
   - Press `Ctrl+B` then `D` to detach (servers keep running)
   - Run `./stop.sh` to stop all servers

   **OR start servers manually:**

4. **Start backend:**
   ```bash
   cd rust
   cargo run
   ```

5. **Start frontend (in new terminal):**
   ```bash
   cd web
   npm run dev
   ```

6. **Open browser:**
   Navigate to `http://localhost:3000`

## Development

### Quick Commands
- `./run.sh` - Start both servers in tmux
- `./stop.sh` - Stop all servers
- `tmux attach -t rogue2-dev` - Reattach to running session

### Frontend
- `npm run dev` - Start dev server
- `npm run build` - Build for production
- `npm run type-check` - Check TypeScript types
- `npm run generate-types` - Regenerate FlatBuffers types

### Backend
- `cargo run` - Run server
- `cargo build --release` - Build for production
- `cargo test` - Run tests
- `cargo check` - Check for errors

### Generate FlatBuffers Types
After modifying `schema/messages.fbs`, run:
```bash
cd schema
./generate.sh
```

## Architecture

- **Frontend:** TypeScript, Vite, bitecs (ECS), WebGL, FlatBuffers
- **Backend:** Rust, Tokio, hecs (ECS), WebSockets, FlatBuffers
- **Protocol:** Binary WebSocket messages via FlatBuffers

## Configuration

### Frontend (.env)
```
VITE_WS_URL=ws://localhost:8080/game
```

### Backend (environment variables)
```
HOST=0.0.0.0
PORT=8080
LOG_LEVEL=info
MAX_PLAYERS=100
```

## Current Status

Phase 1: Project scaffolding complete
- ✅ Directory structure
- ✅ Build configuration
- ✅ FlatBuffers schema
- 🚧 Network layer
- 🚧 ECS implementation
- 🚧 Rendering system
- 🚧 Game loop

See `prompts/setup.md` for full specification.

## Deployment

### Backend Docker
```bash
cd rust
docker build -t game-server .
docker run -p 8080:8080 game-server
```

### Frontend Static Build
```bash
cd web
npm run build
# Deploy ./dist to any static hosting
```

## License

[Add your license here]
