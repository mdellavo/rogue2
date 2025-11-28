# AI Assistant Guide for Multiplayer Roguelike Game

This document provides context and guidelines for AI assistants working on this codebase.

## Project Overview

**Type:** 2D sprite-based multiplayer roguelike action RPG
**Style:** Similar to The Legend of Zelda (top-down view)
**Tech Stack:** TypeScript frontend + Rust backend
**Current Phase:** Phase 1 - Core Multiplayer Foundation (MVP)

## Architecture Summary

### Communication
- **Protocol:** WebSocket with binary FlatBuffers serialization
- **Endpoint:** `ws://localhost:8080/game`
- **Pattern:** Server-authoritative with client-side prediction
- **Updates:** Full snapshot on connect, delta updates at 60 Hz

### Frontend (`./web`)
- **Language:** TypeScript (strict mode, no `any` types)
- **Build Tool:** Vite
- **ECS:** bitecs
- **Rendering:** WebGL (2D sprite rendering)
- **State:** ECS world + separate UI state
- **Tick Rate:** Client predicts immediately, server corrects at 60 Hz

### Backend (`./rust`)
- **Language:** Rust with Tokio async runtime
- **ECS:** hecs
- **Networking:** tokio-tungstenite (WebSockets)
- **Game Loop:** 60 Hz (16.67ms per tick)
- **Storage:** In-memory only (no persistence in MVP)

### Shared (`./schema`)
- **Format:** FlatBuffers (`.fbs` files)
- **Generation:** `./schema/generate.sh` creates types for both languages
- **Messages:** PlayerInput, GameStateDelta, GameStateSnapshot, MapTransition, etc.

## Key Technical Decisions

### 1. **Server Authority**
- Server is the authoritative source for ALL game state
- Client predictions are corrected by server updates
- Never trust client input - validate everything server-side

### 2. **ECS Architecture**
Both frontend and backend use Entity-Component-System:
- **Entities:** Just IDs (numbers)
- **Components:** Data structures (Position, Health, Stats, etc.)
- **Systems:** Logic that operates on components (MovementSystem, AISystem, etc.)

### 3. **Type Safety**
- TypeScript: Strict mode, explicit types everywhere
- FlatBuffers: Generates matching types for both languages
- No `any` types except for external libraries

### 4. **Movement System**
- **Not grid-based** - smooth continuous movement in 2D space
- Position stored as floating-point (x, y) in pixels
- Movement speed: 200 pixels/second
- Tile size: 32x32 pixels

### 5. **Combat System**
- D&D 5e inspired stats (STR, DEX, CON, INT, WIS, CHA)
- Attack roll: d20 + modifier vs AC
- Real-time with cooldowns (not turn-based)
- Melee cooldown: 1 second

### 6. **Map System**
- Chunk-based loading (32x32 tiles per chunk)
- 3x3 chunk grid around player (9 chunks loaded)
- Maps can be static (JSON) or procedurally generated (seeded)
- Each map has background music and optional ambient sound

## Project Structure

```
rogue2/
├── schema/                    # Shared FlatBuffers schemas
│   ├── messages.fbs           # ALL network message definitions
│   └── generate.sh            # Run this after modifying .fbs
├── web/                       # TypeScript Frontend
│   ├── src/
│   │   ├── core/
│   │   │   ├── ecs/          # Components, systems, world
│   │   │   ├── network/      # WebSocket client, message handling
│   │   │   ├── rendering/    # WebGL renderer, camera, sprites
│   │   │   ├── input/        # Keyboard, mouse input
│   │   │   ├── audio/        # AudioManager, music, SFX
│   │   │   └── state/        # Game state, prediction system
│   │   ├── ui/               # UI components, styles
│   │   ├── config.ts         # Configuration constants
│   │   └── main.ts           # Entry point
│   ├── public/
│   │   ├── assets/           # Sprites, music, SFX
│   │   └── manifests/        # sprites.json, audio.json
│   └── tsconfig.json         # Strict TypeScript config
└── rust/                     # Rust Backend
    ├── src/
    │   ├── ecs/              # Components, systems
    │   ├── network/          # WebSocket server, client handling
    │   ├── game/             # Game loop, state, combat
    │   ├── map/              # Map loading, chunk system
    │   ├── ai/               # NPC AI (hostile, friendly)
    │   ├── config.rs         # Configuration from env vars
    │   └── main.rs           # Entry point
    ├── data/maps/            # Map JSON files
    └── Cargo.toml            # Dependencies
```

## Development Guidelines

### When Adding Features

1. **Start with the schema** (`schema/messages.fbs`)
   - Add new message types if needed
   - Run `./schema/generate.sh`

2. **Backend first** (server authority)
   - Implement in `rust/src/`
   - Add to appropriate system (game, ecs, network, etc.)
   - Validate all inputs
   - Update game loop if needed

3. **Frontend second** (display & prediction)
   - Implement in `web/src/`
   - Add to appropriate core system
   - Handle server corrections gracefully

4. **Test multiplayer**
   - Open 2+ browser windows
   - Verify server authority works
   - Check for race conditions

### Code Style

**TypeScript:**
- Use interfaces for data structures
- Use classes for systems/managers
- Explicit return types on all functions
- No `any` types
- Use path aliases: `@/` and `@generated/`

**Rust:**
- Use structs for components
- Use systems as functions or trait implementations
- `anyhow::Result` for error handling
- Prefer explicit types over `let x = ...`

### Common Patterns

**Adding a Component:**
1. Define in `web/src/core/ecs/components.ts`
2. Define in `rust/src/ecs/components.rs`
3. Add to FlatBuffers schema if networked

**Adding a System:**
1. Backend: Create in `rust/src/ecs/systems.rs` or dedicated file
2. Add to game loop in `rust/src/game/loop.rs`
3. Frontend: Create in `web/src/core/ecs/systems.ts`
4. Add to render/update loop in `web/src/main.ts`

**Adding a Message Type:**
1. Define in `schema/messages.fbs`
2. Add to `MessageType` union
3. Run `./schema/generate.sh`
4. Handle in backend: `rust/src/network/messages.rs`
5. Handle in frontend: `web/src/core/network/MessageHandler.ts`

## Important Constants

```typescript
// Frontend (web/src/config.ts)
tickRate: 60           // Server tick rate
inputRate: 20          // Client input send rate
viewportWidth: 800     // Pixels
viewportHeight: 608    // Pixels
tileSize: 32           // Pixels per tile

// Movement
movementSpeed: 200     // Pixels per second

// Distances
visionRange: 20        // Tiles (640 pixels)
proximityChat: 15      // Tiles (480 pixels)
soundRange: 20         // Tiles (640 pixels)

// Combat
meleeCooldown: 1000    // Milliseconds
meleeRange: 1.5        // Tiles (48 pixels)

// Chunks
chunkSize: 32          // Tiles per chunk (1024x1024 pixels)
loadedChunks: 3x3      // 9 chunks around player
```

## Current Phase: Phase 1 (MVP)

### ✅ In Scope
- WebSocket connection and messaging (FlatBuffers)
- Player connection, join, and disconnect
- Smooth continuous movement (WASD controls)
- Single static map (loaded from JSON)
- WebGL rendering with sprites
- Basic melee combat with cooldowns
- D&D-inspired stats and damage calculation
- Simple inventory (pickup, drop items)
- Player death and respawn
- Basic hostile NPCs with simple AI
- Server-authoritative state with client prediction

### ❌ Out of Scope (Future Phases)
- Chunk loading (use small single map for now)
- Procedural generation (static map only)
- Map transitions / doors
- Equipment system (basic weapon only)
- Proximity chat (text chat only)
- Background music and sound effects (architecture exists, assets Phase 4)
- Friendly NPCs / merchants
- Complex AI behaviors

## Common Tasks

### Generate FlatBuffers Types
```bash
cd schema
./schema/generate.sh
```

### Run Tests
```bash
# Frontend type check
cd web && npm run type-check

# Backend check
cd rust && cargo check

# Backend tests (when added)
cd rust && cargo test
```

### Add a New Map
1. Create JSON file in `rust/data/maps/`
2. Follow structure in `prompts/setup.md` (lines 765-777)
3. Include: id, name, width, height, backgroundMusic, ambientSound, tileData, spawnPoints

### Debug Networking
- Check browser DevTools → Network → WS tab
- Backend logs: Set `RUST_LOG=debug`
- Message format issues: Regenerate FlatBuffers types

## Important Constraints

1. **No `any` types** in TypeScript (except external libs)
2. **Server validates everything** - never trust client
3. **60 Hz game loop** - maintain this timing
4. **Delta updates only** - not full state snapshots (except initial connect)
5. **No persistence** - in-memory only for MVP
6. **Single map** - no chunk loading in Phase 1
7. **Type safety** - both languages strongly typed via FlatBuffers

## Performance Targets

- Server tick: 16.67ms (60 Hz)
- Client render: 60 FPS
- Network latency tolerance: 100-200ms
- Max players: 100 (configurable)
- Delta update size: Minimize (only changed entities)

## Helpful References

- Full spec: `./prompts/setup.md`
- Setup guide: `./README.md`
- FlatBuffers docs: https://flatbuffers.dev/
- bitecs docs: https://github.com/NateTheGreatt/bitecs
- hecs docs: https://docs.rs/hecs/

## When in Doubt

1. **Check the spec** (`prompts/setup.md`) - it's the source of truth
2. **Server authority** - if it's game state, server decides
3. **Type safety** - add explicit types, avoid `any`
4. **Validate inputs** - never trust the client
5. **Test multiplayer** - always test with 2+ clients

## Quick Command Reference

```bash
# Generate types
cd schema && ./generate.sh

# Frontend dev
cd web && npm run dev          # Start dev server
cd web && npm run type-check   # Type check
cd web && npm run build        # Production build

# Backend dev
cd rust && cargo run           # Start server
cd rust && cargo check         # Check compilation
cd rust && cargo build --release  # Production build

# Both
# Terminal 1: cd rust && cargo run
# Terminal 2: cd web && npm run dev
# Browser: http://localhost:3000
```

---

**Remember:** This is a multiplayer game. Always think about:
- What happens with 2 players?
- What if they're on different maps/chunks?
- What if one player lags?
- Is the server authoritative?
- Are we sending unnecessary data?

Good luck! 🎮
