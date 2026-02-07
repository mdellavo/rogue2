# Multiplayer Roguelike Game

A 2D sprite-based multiplayer roguelike game with TypeScript frontend and Rust backend.

## Project Structure

- `web/` - TypeScript frontend (Vite + WebGL)
- `rust/` - Rust backend (Tokio + WebSockets)
- `schema/` - FlatBuffers message schemas
- `web/public/assets/tilesets/` - Tileset sprite images
- `web/public/manifests/` - Tileset JSON manifests

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

### Frontend Environment Variables

Create a `.env` file in the `web/` directory:

```bash
# WebSocket server URL
VITE_WS_URL=ws://localhost:8080/game

# Asset server URL for loading tilesets and sprites
# In development, this should point to the Vite dev server
# In production, this should point to your CDN or static file server
VITE_ASSET_SERVER_URL=http://localhost:3000
```

**Asset Loading:**
The game loads sprite tilesets dynamically from the asset server at runtime. Tilesets consist of:
- **Image file**: `${VITE_ASSET_SERVER_URL}/public/assets/tilesets/tileset_${name}.png`
- **Manifest file**: `${VITE_ASSET_SERVER_URL}/public/manifests/tileset_${name}.json`

Default tileset: `tileset_dungeon` (128×32 tiles at 64×64px = 12.9 MB PNG + 1.1 MB JSON)

### Backend Environment Variables
```bash
HOST=0.0.0.0
PORT=8080
LOG_LEVEL=info
MAX_PLAYERS=100

# Optional: Map generation
USE_PROCEDURAL_MAP=true
PROCEDURAL_SEED=12345
PROCEDURAL_WIDTH=100
PROCEDURAL_HEIGHT=100
```

## Current Status

### Core Systems
- ✅ WebSocket networking with FlatBuffers
- ✅ ECS implementation (bitecs + hecs)
- ✅ Chunked map loading (32×32 tiles, 3×3 grid)
- ✅ Dynamic tileset loading with WebGL textures
- ✅ Server-authoritative game loop (60 Hz)
- ✅ Client-side prediction and input handling
- ✅ Player movement and camera following
- ✅ Procedural map generation
- ✅ D&D-inspired character creation

### Rendering Features
- ✅ WebGL 2D sprite rendering
- ✅ Texture-based tileset system
- ✅ Batch rendering with automatic texture switching
- ✅ Viewport culling for performance
- ✅ Health bars and fog of war
- ✅ Fallback colored rendering for missing sprites

See `prompts/setup.md` for full specification.

## Asset Management

### Tileset Format

Tilesets use a two-file format for efficient sprite rendering:

#### 1. Image File (`tileset_${name}.png`)
- Large sprite atlas containing all tiles
- Example: `tileset_dungeon.png` (8192×2048px, 128 cols × 32 rows)
- Each tile: 64×64 pixels
- Format: PNG with transparency

#### 2. Manifest File (`tileset_${name}.json`)
Describes sprite locations and metadata:

```json
{
  "meta": {
    "tileWidth": 64,
    "tileHeight": 64,
    "origin": "top-left",
    "pages": [
      {
        "id": "dungeon",
        "file": "tileset_dungeon.png",
        "cols": 128,
        "rows": 32
      }
    ]
  },
  "tiles": [
    {
      "id": "stone_wall",
      "page": "dungeon",
      "name": "Stone Wall",
      "row": 0,
      "col": 2,
      "x": 128,
      "y": 0,
      "w": 64,
      "h": 64,
      "type": "wall",
      "walkable": false,
      "description": "A solid stone wall"
    }
  ]
}
```

### Adding New Tilesets

1. **Place files in correct directories:**
   ```
   web/public/assets/tilesets/tileset_myname.png
   web/public/manifests/tileset_myname.json
   ```

2. **Load in code:**
   ```typescript
   await tilesetManager.loadTileset('myname');
   renderer.drawSprite('myname', 'sprite_id', x, y, width, height);
   ```

3. **Configure default tileset:**
   Edit `web/src/config.ts`:
   ```typescript
   defaultTileset: 'myname'
   ```

### Asset Server Configuration

**Development:**
- Vite dev server serves assets automatically from `web/public/`
- Set `VITE_ASSET_SERVER_URL=http://localhost:3000`
- Assets accessible at `http://localhost:3000/public/assets/tilesets/...`

**Production:**
- Upload `web/public/` contents to CDN or static file server
- Set `VITE_ASSET_SERVER_URL=https://your-cdn.com`
- Ensure CORS headers allow `*.png` and `*.json` access
- Example headers:
  ```
  Access-Control-Allow-Origin: *
  Access-Control-Allow-Methods: GET, HEAD
  ```

### Performance Considerations

**Tileset Sizes:**
- Small: 32×32 tiles = ~2 MB (characters, items)
- Medium: 64×64 tiles = ~8 MB (terrain, features)
- Large: 128×32 tiles = ~13 MB (comprehensive dungeon tileset)

**Loading Strategy:**
- Default tileset loads on game start (blocking)
- Additional tilesets can lazy-load on demand
- Tilesets cached in memory after first load
- Use `tilesetManager.unloadTileset(name)` to free memory

**Optimization Tips:**
- Use PNG compression (pngquant, oxipng)
- Consider WebP format for smaller files (~30% reduction)
- Enable texture compression on GPU (DXT, ETC2)
- Lazy-load tilesets only when needed
- Unload unused tilesets when switching maps

## Deployment

### Backend Docker
```bash
cd rust
docker build -t game-server .
docker run -p 8080:8080 -e LOG_LEVEL=info game-server
```

### Frontend Static Build

1. **Build the frontend:**
   ```bash
   cd web
   npm run build
   # Output: ./dist directory
   ```

2. **Deploy assets and build:**

   **Option A: Single Server (Simple)**
   ```bash
   # Deploy entire dist directory to static hosting (Netlify, Vercel, etc.)
   # Assets are served from same origin - no CORS issues
   # Set VITE_ASSET_SERVER_URL to your domain (or leave default)
   ```

   **Option B: Separate CDN (Performance)**
   ```bash
   # 1. Deploy dist/public/* to CDN (CloudFront, Cloudflare, etc.)
   #    Example: https://cdn.yourgame.com/public/assets/tilesets/...

   # 2. Deploy dist/* (without public/) to main hosting
   #    Example: https://play.yourgame.com

   # 3. Update environment variable:
   VITE_ASSET_SERVER_URL=https://cdn.yourgame.com

   # 4. Ensure CDN has CORS headers:
   Access-Control-Allow-Origin: https://play.yourgame.com
   Access-Control-Allow-Methods: GET, HEAD
   Cache-Control: public, max-age=31536000
   ```

3. **Verify deployment:**
   ```bash
   # Check asset loading
   curl -I https://your-cdn.com/public/assets/tilesets/tileset_dungeon.png
   # Should return 200 with CORS headers

   # Check manifest loading
   curl https://your-cdn.com/public/manifests/tileset_dungeon.json
   # Should return valid JSON
   ```

### Environment Variables for Production

**Frontend (.env.production):**
```bash
VITE_WS_URL=wss://api.yourgame.com/game
VITE_ASSET_SERVER_URL=https://cdn.yourgame.com
```

**Backend:**
```bash
HOST=0.0.0.0
PORT=8080
LOG_LEVEL=warn
MAX_PLAYERS=1000
USE_PROCEDURAL_MAP=true
PROCEDURAL_WIDTH=200
PROCEDURAL_HEIGHT=200
```

### Checklist for Production Deployment

- [ ] Build frontend with production environment variables
- [ ] Upload tileset assets to CDN or static hosting
- [ ] Configure CORS headers for asset server
- [ ] Test asset loading from production URLs
- [ ] Deploy backend with proper LOG_LEVEL
- [ ] Configure WebSocket URL (wss:// for HTTPS sites)
- [ ] Test with multiple concurrent players
- [ ] Monitor asset loading errors in browser console
- [ ] Set up CDN caching headers for assets
- [ ] Consider using WebP images for smaller file sizes

## License

[Add your license here]
