# Summary

A 2D sprite-based multiplayer roguelike game that will be played in web browsers.

The TypeScript frontend will live in the `./web` directory.

The Rust backend will live in the `./rust` directory.

**Why TypeScript for Frontend:**
- Type safety prevents runtime errors during development
- Better IDE support with autocomplete and refactoring
- Self-documenting code with explicit type annotations
- Easier maintenance and collaboration on large codebase
- Generated types from FlatBuffers schemas ensure frontend/backend compatibility
- Compile-time validation catches bugs before deployment

## Communication Protocol

The frontend and backend communicate via WebSocket:
- **Endpoint:** `ws://localhost:8080/game` (configurable via environment variables)
- **Protocol:** Binary messages serialized with FlatBuffers
- **Heartbeat:** Client sends ping every 5 seconds, server responds with pong
- **Reconnection:** Client attempts exponential backoff reconnection (1s, 2s, 4s, 8s, max 30s)
- **Initial Connection:** Server sends full game state snapshot
- **Ongoing Updates:** Server sends delta updates only (changed entities/components)

The frontend sends user input messages to the backend. The backend processes inputs, updates the authoritative game state, and broadcasts state deltas to all connected clients.

**Server Authority:** The server is the authoritative source for all game state. Client predictions are corrected by server updates.

## Message Format

Messages are binary format serialized with FlatBuffers. Message types include:
- Client → Server: `PlayerInput`, `ChatMessage`, `Ping`, `InteractDoor`
- Server → Client: `GameStateDelta`, `GameStateSnapshot`, `MapTransition`, `SystemMessage`, `Pong`

**FlatBuffers Schema & Type Generation:**
- Define message schemas in `.fbs` files (shared between frontend and backend)
- Use `flatc` compiler to generate TypeScript types for frontend
- Use `flatc` compiler to generate Rust types for backend
- Generated TypeScript types provide full type safety for all network messages
- Example command: `flatc --ts -o ./web/src/generated schema.fbs`

**MapTransition Message:**
When a player transitions to a new map, the server sends:
```typescript
{
  mapId: string,
  mapName: string,
  position: { x: number, y: number },
  backgroundMusic: string,  // Music track ID from audio manifest
  ambientSound: string | null,  // Optional ambient sound ID
  chunkData: ChunkData[],  // Initial chunk data for new map (strongly typed)
}
```

## Game State

Game state includes:
- **Maps:** Tile data, terrain types, dimensions, chunk metadata
- **Players:** Position (x, y), velocity, health, stats (STR, DEX, CON, INT, WIS, CHA), inventory, equipment
- **NPCs:** Position, health, stats, inventory, equipment, AI state
- **Objects:** Position, type (chest, door, sign, etc.), properties, interaction state
- **System Messages:** Chat, notifications, combat logs 

## Asset Systems

### Sprite System

Game sprites will be loaded as PNG images. Sprites will be defined in a JSON manifest matching the following schema:

```json
{
  "meta": {
    "tileWidth": 32,
    "tileHeight": 32,
    "origin": "top-left",
    "pages": [
      {
        "id": "dungeon",
        "file": "tileset_dungeon.png",
        "cols": 16,
        "rows": 16
      },
      {
        "id": "characters",
        "file": "tileset_characters.png",
        "cols": 8,
        "rows": 8
      }
    ],
    "note": "x,y are computed as col*tileWidth, row*tileHeight on each page."
  },
  "tiles": [
    {
      "id": "stone_floor",
      "page": "dungeon",
      "name": "Stone Floor",
      "row": 0,
      "col": 0,
      "x": 0,
      "y": 0,
      "w": 32,
      "h": 32,
      "type": "floor",
      "walkable": true,
      "description": "Basic dungeon floor tile"
    },
    {
      "id": "stone_wall",
      "page": "dungeon",
      "name": "Stone Wall",
      "row": 0,
      "col": 1,
      "x": 32,
      "y": 0,
      "w": 32,
      "h": 32,
      "type": "wall",
      "walkable": false,
      "description": "Solid stone wall"
    },
    {
      "id": "wooden_door",
      "page": "dungeon",
      "name": "Wooden Door",
      "row": 0,
      "col": 2,
      "x": 64,
      "y": 0,
      "w": 32,
      "h": 32,
      "type": "door",
      "walkable": false,
      "interactable": true,
      "description": "Door leading to another area"
    }
  ]
}
```

### Music & Audio System

Background music and ambient sounds will be loaded as audio files (OGG Vorbis primary format, MP3 fallback for compatibility). Audio assets will be defined in a JSON manifest matching the following schema:

```json
{
  "music": [
    {
      "id": "overworld_theme",
      "name": "Overworld Theme",
      "file": "music/overworld.ogg",
      "fallback": "music/overworld.mp3",
      "loop": true,
      "loopStart": 0.0,
      "loopEnd": null,
      "volume": 0.7,
      "description": "Main overworld exploration music"
    },
    {
      "id": "dungeon_ambient",
      "name": "Dungeon Ambience",
      "file": "music/dungeon_dark.ogg",
      "fallback": "music/dungeon_dark.mp3",
      "loop": true,
      "loopStart": 0.0,
      "loopEnd": null,
      "volume": 0.6,
      "description": "Dark atmospheric dungeon music"
    },
    {
      "id": "combat_theme",
      "name": "Combat Music",
      "file": "music/battle.ogg",
      "fallback": "music/battle.mp3",
      "loop": true,
      "loopStart": 4.5,
      "loopEnd": 60.0,
      "volume": 0.8,
      "description": "Intense combat music with intro"
    }
  ],
  "ambientSounds": [
    {
      "id": "cave_drips",
      "name": "Cave Water Drips",
      "file": "ambient/cave_drips.ogg",
      "fallback": "ambient/cave_drips.mp3",
      "loop": true,
      "volume": 0.4,
      "description": "Water dripping in caves"
    },
    {
      "id": "forest_birds",
      "name": "Forest Birds",
      "file": "ambient/forest_ambience.ogg",
      "fallback": "ambient/forest_ambience.mp3",
      "loop": true,
      "volume": 0.5,
      "description": "Birds chirping in forest"
    }
  ],
  "soundEffects": [
    {
      "id": "sword_swing",
      "name": "Sword Swing",
      "file": "sfx/sword_swing.ogg",
      "fallback": "sfx/sword_swing.mp3",
      "loop": false,
      "volume": 0.6,
      "description": "Sword attack sound"
    },
    {
      "id": "door_open",
      "name": "Door Open",
      "file": "sfx/door_open.ogg",
      "fallback": "sfx/door_open.mp3",
      "loop": false,
      "volume": 0.7,
      "description": "Door opening sound"
    },
    {
      "id": "item_pickup",
      "name": "Item Pickup",
      "file": "sfx/item_pickup.ogg",
      "fallback": "sfx/item_pickup.mp3",
      "loop": false,
      "volume": 0.5,
      "description": "Item pickup sound"
    }
  ]
}
```

## Persistence & Authentication

Currently, game state exists in memory only. No player authentication is needed. Players are identified by their WebSocket connection. When a player disconnects, their character remains in the game for 30 seconds before being removed.

# Game Overview

The game is a roguelike action RPG visually similar to The Legend of Zelda. Players start on an overworld map and explore caves, dungeons, mazes, and other areas. Each map has doors/portals that lead to different maps. Maps are both procedurally generated and statically loaded from data files. Maps contain interactive objects: trees, rocks, chests, signs, buildings, NPCs, etc.

## Map System

**Chunk Loading:**
- Maps are divided into chunks of 32x32 tiles (1024x1024 pixels)
- Chunks are loaded based on player proximity (3x3 chunk grid around player = 9 chunks loaded)
- Server tracks which chunks each client has loaded
- Server only sends entity updates for entities within loaded chunks
- Chunk transitions are seamless; entities can move freely across chunk boundaries

**Map Types:**
- **Static maps:** Loaded from JSON files, designed by hand
- **Procedural maps:** Generated using seeded random generation (seed stored in map metadata)
- **Persistence:** Procedurally generated maps are generated once per server lifetime, then kept in memory

**Map Data Structure:**
Each map includes the following metadata:
- `id`: Unique map identifier (e.g., "overworld", "dungeon_01")
- `name`: Human-readable name
- `width`, `height`: Map dimensions in tiles
- `tileData`: 2D array of tile IDs
- `spawnPoints`: Array of safe spawn locations
- `backgroundMusic`: Music track ID to play (e.g., "overworld_theme", "dungeon_ambient")
- `ambientSound`: Optional ambient sound loop ID (e.g., "cave_drips", "forest_birds")

**Map Transitions:**
- Doors/portals have a `targetMap` and `targetPosition` property
- Interacting with a door (spacebar/click) triggers a map transition
- Client sends `InteractDoor` message, server validates and sends `MapTransition` message
- Server responds with new map metadata including `backgroundMusic` ID
- Client unloads old map chunks and loads new map chunks
- Client crossfades to new background music track (2 second fade)

## Stat & Combat System

**D&D-Inspired Stats:**
- Uses simplified D&D 5th Edition core stats: STR, DEX, CON, INT, WIS, CHA
- Stats range from 1-20 (10 is average)
- Stat modifiers: `(stat - 10) / 2` rounded down
- Combat uses: Attack roll (d20 + modifier) vs Armor Class (AC)
- Damage: Weapon dice + STR/DEX modifier
- No skill checks, saving throws, or magic system in MVP (future feature)

### Character Species & Classes

**Character Creation:**
Players choose a species (race) and class when first joining the server. This determines their starting stats, abilities, and playstyle. Species and class choices are permanent for that character.

**Base Stats:**
All characters start with base stats of 10 in each attribute. Species and class provide bonuses/penalties to these base stats.

**Character Species:**

1. **Human**
   - Stat bonuses: +1 to all stats
   - Base HP: 10
   - Movement speed: 200 pixels/second (normal)
   - Trait: *Versatile* - Gain 10% more experience from all sources
   - Description: Adaptable and ambitious, humans are the most common species

2. **Elf**
   - Stat bonuses: +2 DEX, +1 INT, -1 CON
   - Base HP: 8
   - Movement speed: 220 pixels/second (+10% faster)
   - Trait: *Keen Senses* - Vision range increased to 24 tiles (768px)
   - Description: Graceful and perceptive, elves excel at ranged combat and magic

3. **Dwarf**
   - Stat bonuses: +2 CON, +1 STR, -1 DEX
   - Base HP: 12
   - Movement speed: 180 pixels/second (-10% slower)
   - Trait: *Stone's Endurance* - Damage resistance: reduce all incoming damage by 1 (minimum 1 damage)
   - Description: Sturdy and resilient, dwarves are tough melee fighters

4. **Halfling**
   - Stat bonuses: +2 DEX, +1 CHA, -1 STR
   - Base HP: 8
   - Movement speed: 190 pixels/second (-5% slower)
   - Trait: *Lucky* - When you roll a 1 on attack or damage, reroll once (must use new roll)
   - Description: Small and nimble, halflings rely on agility and luck

5. **Half-Orc**
   - Stat bonuses: +2 STR, +1 CON, -1 INT
   - Base HP: 12
   - Movement speed: 210 pixels/second (+5% faster)
   - Trait: *Relentless Endurance* - When reduced to 0 HP, drop to 1 HP instead (once per respawn)
   - Description: Powerful and fierce, half-orcs are devastating in melee combat

6. **Gnome**
   - Stat bonuses: +2 INT, +1 DEX, -1 STR
   - Base HP: 8
   - Movement speed: 180 pixels/second (-10% slower)
   - Trait: *Gnome Cunning* - Automatically succeed on saves against magic effects (future: when magic is implemented)
   - Description: Small and intelligent, gnomes make excellent spellcasters

**Character Classes:**

1. **Fighter**
   - Primary stats: STR, CON
   - Class bonuses: +2 STR, +1 CON, +2 HP
   - Starting AC: 16 (chain mail)
   - Starting weapon: Longsword (1d8 damage)
   - Attack cooldown: 0.8 seconds (20% faster)
   - Class ability: *Second Wind* - Heal 25% of max HP (cooldown: 60 seconds)
   - Description: Masters of martial combat who excel with melee weapons

2. **Rogue**
   - Primary stats: DEX, CHA
   - Class bonuses: +2 DEX, +1 CHA
   - Starting AC: 14 (leather armor)
   - Starting weapon: Dagger (1d4 damage, can dual wield)
   - Attack cooldown: 0.7 seconds (30% faster)
   - Class ability: *Sneak Attack* - First attack on an enemy deals double damage (cooldown: 10 seconds)
   - Description: Stealthy and cunning, rogues deal high burst damage

3. **Cleric**
   - Primary stats: WIS, CON
   - Class bonuses: +2 WIS, +1 CON, +1 HP
   - Starting AC: 15 (scale mail)
   - Starting weapon: Mace (1d6 damage)
   - Attack cooldown: 1.0 seconds (normal)
   - Class ability: *Healing Word* - Heal nearby ally for 30% of their max HP (cooldown: 30 seconds, range: 10 tiles)
   - Description: Divine spellcasters who can heal allies and harm foes

4. **Wizard**
   - Primary stats: INT, WIS
   - Class bonuses: +2 INT, +1 WIS, -2 HP
   - Starting AC: 11 (cloth robes)
   - Starting weapon: Staff (1d4 damage)
   - Attack cooldown: 1.2 seconds (20% slower)
   - Class ability: *Magic Missile* - Fire 3 projectiles that auto-hit for 1d4+INT each (cooldown: 15 seconds, range: 15 tiles)
   - Description: Powerful spellcasters with devastating ranged magic

5. **Ranger**
   - Primary stats: DEX, WIS
   - Class bonuses: +2 DEX, +1 WIS, +1 HP
   - Starting AC: 14 (leather armor)
   - Starting weapon: Shortbow (1d6 damage, ranged)
   - Attack cooldown: 1.0 seconds (normal)
   - Class ability: *Hunter's Mark* - Mark an enemy; deal +1d6 damage to them for 30 seconds (cooldown: 45 seconds)
   - Description: Expert trackers and archers who excel at ranged combat

6. **Barbarian**
   - Primary stats: STR, CON
   - Class bonuses: +2 STR, +2 CON, +4 HP
   - Starting AC: 13 (no armor, CON-based)
   - Starting weapon: Greataxe (1d12 damage)
   - Attack cooldown: 1.1 seconds (10% slower)
   - Class ability: *Rage* - Deal +2 damage and take 50% reduced damage for 10 seconds (cooldown: 60 seconds)
   - Description: Ferocious warriors who channel raw fury in combat

**Character Progression:**
- Characters gain experience from defeating enemies and completing objectives
- Every level gained grants: +1 to two stats of choice, +5 HP, improved class abilities (future)
- Max level: 20 (can reach maximum 20 in all stats)
- Level 1: 0 XP, Level 2: 300 XP, Level 3: 900 XP, Level 4: 2700 XP (exponential curve)

**Example Character:**
- Species: Elf (DEX +2, INT +1, CON -1)
- Class: Ranger (DEX +2, WIS +1, HP +1)
- Final starting stats: STR 10, DEX 14, CON 9, INT 11, WIS 11, CHA 10
- Starting HP: 8 (elf) + 1 (ranger) = 9 HP
- Starting AC: 14 (leather armor)
- Movement speed: 220 pixels/second
- Weapon: Shortbow (1d6 ranged)
- Abilities: Keen Senses (vision +4 tiles), Hunter's Mark (bonus damage)

### Monsters & Creatures

**Monster Categories:**
Monsters are categorized by Challenge Rating (CR), which determines their difficulty and XP rewards. Monsters have the same stat system as players (STR, DEX, CON, INT, WIS, CHA) and use similar combat mechanics.

**Monster Behavior:**
- **Passive:** Does not initiate combat, flees when attacked
- **Neutral:** Ignores players unless attacked, then becomes hostile
- **Hostile:** Aggros players on sight within detection range
- **Pack:** Calls nearby allies when entering combat
- **Boss:** Special encounters with unique mechanics

**Level 1 Monsters (CR 1/4 - 1/2):**

1. **Giant Rat**
   - CR: 1/4 | Level: 1 | XP: 50
   - HP: 7 (2d6) | AC: 12
   - Stats: STR 8, DEX 15, CON 11, INT 2, WIS 10, CHA 4
   - Speed: 200 px/s
   - Attack: Bite (1d4 piercing, 1.0s cooldown)
   - Behavior: Hostile, pack creature (summons 1-2 nearby rats when attacked)
   - Detection range: 12 tiles (384px)
   - Loot: 60% Nothing, 30% Rat Meat (food), 10% Rat Pelt (crafting)
   - Spawn: Sewers, caves, abandoned buildings

2. **Goblin**
   - CR: 1/4 | Level: 1 | XP: 50
   - HP: 7 (2d6) | AC: 13 (leather armor)
   - Stats: STR 8, DEX 14, CON 10, INT 10, WIS 8, CHA 8
   - Speed: 180 px/s
   - Attack: Rusty Dagger (1d4+1 piercing, 1.0s cooldown)
   - Behavior: Hostile, pack creature
   - Detection range: 15 tiles (480px)
   - Loot: 40% 1-5 copper, 30% Rusty Dagger, 20% Healing Potion (minor), 10% Goblin Ear (quest item)
   - Spawn: Forest, caves, ruins

3. **Skeleton**
   - CR: 1/4 | Level: 1 | XP: 50
   - HP: 13 (2d8+4) | AC: 13 (armor scraps)
   - Stats: STR 10, DEX 14, CON 15, INT 6, WIS 8, CHA 5
   - Speed: 160 px/s
   - Attack: Shortsword (1d6 slashing, 1.2s cooldown)
   - Trait: *Undead Resilience* - Immune to poison, takes double damage from holy attacks
   - Behavior: Hostile
   - Detection range: 18 tiles (576px)
   - Loot: 50% Bone Shard, 30% Rusty Shortsword, 15% Ancient Coin, 5% Skeleton Key
   - Spawn: Crypts, graveyards, ruins

4. **Wolf**
   - CR: 1/4 | Level: 1 | XP: 50
   - HP: 11 (2d8+2) | AC: 13
   - Stats: STR 12, DEX 15, CON 12, INT 3, WIS 12, CHA 6
   - Speed: 240 px/s
   - Attack: Bite (1d6+1 piercing, 0.9s cooldown)
   - Trait: *Pack Tactics* - Deal +1d4 damage when allied wolf is within 2 tiles of target
   - Behavior: Hostile, pack creature (2-4 wolves spawn together)
   - Detection range: 20 tiles (640px) - keen senses
   - Loot: 70% Wolf Pelt, 20% Wolf Meat, 10% Wolf Fang
   - Spawn: Forest, mountains, plains

**Level 2-3 Monsters (CR 1 - 2):**

5. **Orc Warrior**
   - CR: 1 | Level: 2 | XP: 100
   - HP: 15 (2d8+6) | AC: 13 (hide armor)
   - Stats: STR 16, DEX 12, CON 16, INT 7, WIS 11, CHA 10
   - Speed: 200 px/s
   - Attack: Greataxe (1d12+3 slashing, 1.3s cooldown)
   - Trait: *Aggressive* - Move up to 220 px/s when charging enemies
   - Behavior: Hostile, pack (groups of 2-3)
   - Detection range: 16 tiles (512px)
   - Loot: 60% 5-15 copper, 30% Greataxe, 20% Hide Armor, 10% Orcish Trinket
   - Spawn: Orc camps, mountains, badlands

6. **Zombie**
   - CR: 1/4 | Level: 2 | XP: 100
   - HP: 22 (3d8+9) | AC: 8
   - Stats: STR 13, DEX 6, CON 16, INT 3, WIS 6, CHA 5
   - Speed: 120 px/s (very slow)
   - Attack: Slam (1d6+1 bludgeoning, 1.5s cooldown)
   - Trait: *Undead Fortitude* - When reduced to 0 HP, 50% chance to drop to 1 HP instead (once per zombie)
   - Behavior: Hostile, slow but relentless
   - Detection range: 10 tiles (320px)
   - Loot: 60% Rotten Flesh, 20% Tattered Cloth, 15% Copper Coins, 5% Brain (alchemy)
   - Spawn: Graveyards, ruins, plague zones

7. **Giant Spider**
   - CR: 1 | Level: 2 | XP: 100
   - HP: 26 (4d10+4) | AC: 14
   - Stats: STR 14, DEX 16, CON 12, INT 2, WIS 11, CHA 4
   - Speed: 200 px/s, can walk on walls/ceilings
   - Attack: Bite (1d6+2 piercing + 2d8 poison, 1.0s cooldown)
   - Trait: *Web* - 30% chance on hit to reduce target speed by 50% for 3 seconds
   - Behavior: Hostile, ambush predator
   - Detection range: 14 tiles (448px)
   - Loot: 80% Spider Silk, 40% Poison Gland, 20% Giant Fang, 10% Spider Egg
   - Spawn: Caves, forests, ruins

8. **Hobgoblin**
   - CR: 1/2 | Level: 3 | XP: 200
   - HP: 11 (2d8+2) | AC: 16 (chain mail)
   - Stats: STR 13, DEX 12, CON 12, INT 10, WIS 10, CHA 9
   - Speed: 200 px/s
   - Attack: Longsword (1d8+1 slashing, 1.0s cooldown)
   - Trait: *Martial Advantage* - Deal +2d6 damage when allied hobgoblin is within 2 tiles of target
   - Behavior: Hostile, tactical fighter (uses formations)
   - Detection range: 18 tiles (576px)
   - Loot: 70% 10-30 copper, 40% Longsword, 30% Chain Mail, 20% Military Badge, 10% Silver Coin
   - Spawn: Goblin fortresses, bandit camps, military ruins

**Level 4-5 Monsters (CR 3 - 5):**

9. **Ogre**
   - CR: 2 | Level: 4 | XP: 400
   - HP: 59 (7d10+21) | AC: 11 (hide armor)
   - Stats: STR 19, DEX 8, CON 16, INT 5, WIS 7, CHA 7
   - Speed: 220 px/s
   - Attack: Greatclub (2d8+4 bludgeoning, 1.8s cooldown)
   - Trait: *Powerful Build* - Knockback enemies 2 tiles on hit
   - Behavior: Hostile, solitary or pairs
   - Detection range: 14 tiles (448px)
   - Loot: 80% 20-50 copper, 40% Greatclub, 30% Giant's Toe, 20% Ogre Hide, 10% 1-2 silver
   - Spawn: Mountains, caves, bridges (toll collector)

10. **Wight**
   - CR: 3 | Level: 5 | XP: 800
   - HP: 45 (6d8+18) | AC: 14 (studded leather)
   - Stats: STR 15, DEX 14, CON 16, INT 10, WIS 13, CHA 15
   - Speed: 200 px/s
   - Attack: Longsword (1d8+2 slashing, 1.0s cooldown) + Life Drain (1d6 necrotic, heal wight)
   - Trait: *Sunlight Sensitivity* - Take +50% damage during daytime (future: day/night cycle)
   - Trait: *Undead Commander* - Can command nearby skeletons/zombies (summons 2d4 skeletons when combat starts)
   - Behavior: Hostile, commander
   - Detection range: 20 tiles (640px)
   - Loot: 90% 30-80 copper, 50% Enchanted Longsword, 30% Wight Crown, 20% Soul Gem, 10% 1d4 silver
   - Spawn: Crypts, barrows, haunted castles

11. **Troll**
   - CR: 5 | Level: 5 | XP: 800
   - HP: 84 (8d10+40) | AC: 15 (natural armor)
   - Stats: STR 18, DEX 13, CON 20, INT 7, WIS 9, CHA 7
   - Speed: 200 px/s
   - Attack: Claw (1d6+4 slashing, 0.8s cooldown, can attack twice per cooldown)
   - Trait: *Regeneration* - Heal 10 HP per second while above 0 HP (disabled for 1 round if dealt fire/acid damage)
   - Trait: *Keen Smell* - Detection range increased to 25 tiles
   - Behavior: Hostile, aggressive
   - Detection range: 25 tiles (800px)
   - Loot: 90% Troll Blood (alchemy), 60% Troll Hide, 40% 50-100 copper, 20% Troll Heart, 10% 1d6 silver
   - Spawn: Swamps, mountains, caves

**Boss Monsters (CR 6+):**

12. **Young Dragon**
   - CR: 8 | Level: 8 | XP: 2000
   - HP: 178 (17d10+85) | AC: 18 (natural armor)
   - Stats: STR 19, DEX 10, CON 21, INT 12, WIS 11, CHA 15
   - Speed: 220 px/s, flying
   - Attack: Bite (2d10+4 piercing, 1.5s cooldown) or Claw (2d6+4 slashing, 1.0s cooldown)
   - Special: *Fire Breath* - 30 tiles cone, 8d6 fire damage (40s cooldown)
   - Trait: *Dragon Scales* - Resistance to non-magical damage (50% reduction)
   - Trait: *Frightful Presence* - Enemies within 10 tiles take -2 to attack rolls
   - Behavior: Boss, guards treasure hoard
   - Detection range: 30 tiles (960px)
   - Loot: Always 100-500 gold, 2d4 gems, dragon scales, 50% magic item, 25% rare weapon/armor
   - Spawn: Dragon lair (unique location)

13. **Lich**
   - CR: 10 | Level: 10 | XP: 3000
   - HP: 135 (18d8+54) | AC: 17 (natural armor)
   - Stats: STR 11, DEX 16, CON 16, INT 20, WIS 14, CHA 16
   - Speed: 180 px/s, levitating
   - Attack: Touch (3d6 necrotic, 1.0s cooldown, heals lich for half damage dealt)
   - Special: *Ray of Frost* - 20 tiles range, 4d8 cold damage + slow 50% (8s cooldown)
   - Special: *Summon Undead* - Summons 2d6 skeletons/zombies (60s cooldown)
   - Trait: *Magic Resistance* - 50% chance to ignore magical effects
   - Trait: *Phylactery* - If killed, respawns after 7 days unless phylactery destroyed
   - Behavior: Boss, powerful spellcaster
   - Detection range: 35 tiles (1120px)
   - Loot: Always 200-800 gold, Spellbook, Lich Crown, Staff of Power, 75% 2d4 magic items
   - Spawn: Ancient tower/dungeon (unique location)

14. **Demon Lord**
   - CR: 12 | Level: 12 | XP: 5000
   - HP: 262 (21d10+147) | AC: 19 (natural armor)
   - Stats: STR 22, DEX 15, CON 25, INT 16, WIS 13, CHA 18
   - Speed: 240 px/s, flying, teleportation
   - Attack: Greatsword (2d6+6 slashing + 2d6 fire, 1.2s cooldown)
   - Special: *Hellfire Wave* - 15 tiles radius, 10d6 fire damage (60s cooldown)
   - Special: *Teleport Strike* - Teleport to target and attack (20s cooldown)
   - Trait: *Demon Resilience* - Resistance to all damage except holy (50% reduction)
   - Trait: *Aura of Terror* - Enemies within 8 tiles deal 50% reduced damage
   - Behavior: Boss, end-game challenge
   - Detection range: 40 tiles (1280px)
   - Loot: Always 500-2000 gold, Demon Heart, Infernal Blade, 90% 3d6 magic items, unique artifact
   - Spawn: Hell portal (special event location)

**Monster Spawning Rules:**
- Monsters spawn at designated spawn points on each map
- Spawn rate: 1-4 monsters per spawn point every 2-5 minutes
- Boss monsters spawn once per server restart or after 24 hour cooldown
- Spawn chance reduced when players nearby (prevents farming)
- Pack creatures spawn together (2-4 members)
- Maximum monsters per map area: 20 (prevents over-spawning)

**Monster AI:**
- **Patrol:** Wander within 10 tiles of spawn point when not in combat
- **Chase:** Pursue target when hostile, up to 30 tiles from spawn before resetting
- **Flee:** Low HP monsters (<25%) may flee from combat
- **Call for Help:** Pack creatures alert nearby allies when entering combat
- **Target Priority:** Lowest HP target, then nearest target
- **Cooldown AI:** Use abilities on cooldown, basic attacks otherwise

**Monster Loot Tables:**
- Loot is rolled when monster dies
- Common drops: Copper/silver coins, crafting materials
- Uncommon drops: Weapons, armor, potions
- Rare drops: Quest items, gems, magic items
- Boss drops: Always include gold, high chance for magic items
- Players must be within 30 tiles when monster dies to receive XP

### Equipment System

**Equipment Slots:**
Players can equip items in the following slots:
- **Main Hand:** Weapon (melee or ranged)
- **Off Hand:** Shield, second weapon (for dual wielding), or two-handed weapon
- **Armor:** Body armor (cloth, leather, chain, plate)
- **Helmet:** Head protection
- **Accessory 1:** Ring, amulet, or trinket
- **Accessory 2:** Ring, amulet, or trinket

**Item Rarity:**
Items come in different rarity tiers that affect their stats and value:
- **Common** (White): Standard items, no bonuses
- **Uncommon** (Green): +1 bonus or minor enchantment, 2x value
- **Rare** (Blue): +2 bonus or moderate enchantment, 5x value
- **Epic** (Purple): +3 bonus or powerful enchantment, 10x value
- **Legendary** (Orange): +4 bonus or unique enchantment, 50x value

**Weapons:**

**One-Handed Melee Weapons:**
1. **Dagger**
   - Damage: 1d4
   - Attack speed: 0.7s cooldown (fast)
   - Range: 1.0 tiles
   - Weight: Light
   - Properties: Finesse (use DEX instead of STR for attack/damage)
   - Can dual wield
   - Value: 2 gold

2. **Shortsword**
   - Damage: 1d6
   - Attack speed: 0.9s cooldown
   - Range: 1.2 tiles
   - Weight: Light
   - Properties: Finesse
   - Value: 10 gold

3. **Longsword**
   - Damage: 1d8
   - Attack speed: 1.0s cooldown (normal)
   - Range: 1.5 tiles
   - Weight: Medium
   - Properties: Versatile (1d10 if wielded two-handed)
   - Value: 15 gold

4. **Mace**
   - Damage: 1d6
   - Attack speed: 1.0s cooldown
   - Range: 1.3 tiles
   - Weight: Medium
   - Properties: Bludgeoning damage (effective vs skeletons)
   - Value: 12 gold

5. **Battleaxe**
   - Damage: 1d8
   - Attack speed: 1.1s cooldown
   - Range: 1.4 tiles
   - Weight: Medium
   - Properties: Versatile (1d10 two-handed)
   - Value: 18 gold

6. **Warhammer**
   - Damage: 1d8
   - Attack speed: 1.2s cooldown (slower)
   - Range: 1.3 tiles
   - Weight: Heavy
   - Properties: Bludgeoning, +2 damage vs armored enemies (AC 15+)
   - Value: 20 gold

**Two-Handed Melee Weapons:**
7. **Greatsword**
   - Damage: 2d6
   - Attack speed: 1.3s cooldown (slow)
   - Range: 2.0 tiles
   - Weight: Heavy
   - Properties: Two-handed, reach
   - Value: 50 gold

8. **Greataxe**
   - Damage: 1d12
   - Attack speed: 1.4s cooldown (very slow)
   - Range: 1.8 tiles
   - Weight: Heavy
   - Properties: Two-handed, high damage variance
   - Value: 45 gold

9. **Maul**
   - Damage: 2d6
   - Attack speed: 1.5s cooldown (very slow)
   - Range: 1.6 tiles
   - Weight: Heavy
   - Properties: Two-handed, bludgeoning, knockback 1 tile
   - Value: 55 gold

10. **Quarterstaff**
    - Damage: 1d6 (1d8 two-handed)
    - Attack speed: 0.9s cooldown
    - Range: 1.8 tiles
    - Weight: Light
    - Properties: Versatile, reach, can be wielded one-handed
    - Value: 5 gold

**Ranged Weapons:**
11. **Shortbow**
    - Damage: 1d6
    - Attack speed: 1.0s cooldown
    - Range: 15 tiles (480px)
    - Weight: Light
    - Properties: Two-handed, requires arrows, DEX-based
    - Value: 25 gold

12. **Longbow**
    - Damage: 1d8
    - Attack speed: 1.2s cooldown
    - Range: 20 tiles (640px)
    - Weight: Medium
    - Properties: Two-handed, requires arrows, DEX-based, STR 13 required
    - Value: 50 gold

13. **Crossbow (Light)**
    - Damage: 1d8
    - Attack speed: 1.5s cooldown
    - Range: 16 tiles (512px)
    - Weight: Medium
    - Properties: Two-handed, requires bolts, loading (can't attack twice in same turn)
    - Value: 35 gold

14. **Crossbow (Heavy)**
    - Damage: 1d10
    - Attack speed: 2.0s cooldown
    - Range: 18 tiles (576px)
    - Weight: Heavy
    - Properties: Two-handed, requires bolts, loading, STR 15 required
    - Value: 75 gold

**Magic Weapons (Spellcasting Focus):**
15. **Wand**
    - Damage: 1d4 (force damage)
    - Attack speed: 1.0s cooldown
    - Range: 12 tiles (384px)
    - Weight: Light
    - Properties: Magic damage, INT-based, ignores physical armor
    - Value: 30 gold

16. **Staff**
    - Damage: 1d6 (1d8 two-handed)
    - Attack speed: 1.2s cooldown
    - Range: 10 tiles or 1.8 tiles melee
    - Weight: Medium
    - Properties: Versatile, magic damage, INT-based, can melee attack
    - Value: 40 gold

**Armor:**

**Light Armor (DEX bonus applies fully):**
1. **Cloth Robes**
   - AC: 11 + DEX modifier
   - Movement penalty: None
   - Weight: 4 lbs
   - STR requirement: None
   - Properties: No penalty, mage armor
   - Value: 5 gold

2. **Padded Armor**
   - AC: 11 + DEX modifier
   - Movement penalty: None
   - Weight: 8 lbs
   - STR requirement: None
   - Properties: Disadvantage on stealth (noisy)
   - Value: 10 gold

3. **Leather Armor**
   - AC: 12 + DEX modifier
   - Movement penalty: None
   - Weight: 10 lbs
   - STR requirement: None
   - Properties: Common adventurer armor
   - Value: 15 gold

4. **Studded Leather**
   - AC: 13 + DEX modifier
   - Movement penalty: None
   - Weight: 13 lbs
   - STR requirement: None
   - Properties: Reinforced leather
   - Value: 45 gold

**Medium Armor (DEX bonus capped at +2):**
5. **Hide Armor**
   - AC: 13 + DEX modifier (max +2)
   - Movement penalty: -5% speed
   - Weight: 12 lbs
   - STR requirement: None
   - Properties: Crude but effective
   - Value: 15 gold

6. **Chain Shirt**
   - AC: 14 + DEX modifier (max +2)
   - Movement penalty: -5% speed
   - Weight: 20 lbs
   - STR requirement: None
   - Properties: Balanced protection
   - Value: 50 gold

7. **Scale Mail**
   - AC: 15 + DEX modifier (max +2)
   - Movement penalty: -10% speed
   - Weight: 45 lbs
   - STR requirement: STR 12
   - Properties: Disadvantage on stealth
   - Value: 75 gold

8. **Breastplate**
   - AC: 15 + DEX modifier (max +2)
   - Movement penalty: -5% speed
   - Weight: 20 lbs
   - STR requirement: None
   - Properties: Solid torso protection
   - Value: 400 gold

**Heavy Armor (No DEX bonus):**
9. **Ring Mail**
   - AC: 14
   - Movement penalty: -15% speed
   - Weight: 40 lbs
   - STR requirement: STR 13
   - Properties: Disadvantage on stealth, outdated design
   - Value: 30 gold

10. **Chain Mail**
    - AC: 16
    - Movement penalty: -15% speed
    - Weight: 55 lbs
    - STR requirement: STR 13
    - Properties: Disadvantage on stealth, standard heavy armor
    - Value: 75 gold

11. **Splint Armor**
    - AC: 17
    - Movement penalty: -20% speed
    - Weight: 60 lbs
    - STR requirement: STR 15
    - Properties: Disadvantage on stealth, plates and chain
    - Value: 200 gold

12. **Plate Armor**
    - AC: 18
    - Movement penalty: -20% speed
    - Weight: 65 lbs
    - STR requirement: STR 15
    - Properties: Disadvantage on stealth, best physical protection
    - Value: 1500 gold

**Shields:**
1. **Buckler**
   - AC Bonus: +1
   - Weight: 3 lbs
   - Properties: Can attack with one-handed weapon while equipped
   - Value: 5 gold

2. **Shield (Standard)**
   - AC Bonus: +2
   - Weight: 6 lbs
   - Properties: Standard protection
   - Value: 10 gold

3. **Tower Shield**
   - AC Bonus: +3
   - Weight: 15 lbs
   - STR requirement: STR 15
   - Properties: Heavy, provides cover, -10% movement speed
   - Value: 50 gold

**Helmets:**
1. **Cloth Cap**
   - AC Bonus: +0
   - Properties: No protection, cosmetic
   - Value: 1 gold

2. **Leather Cap**
   - AC Bonus: +1
   - Properties: Light protection
   - Value: 5 gold

3. **Chain Coif**
   - AC Bonus: +1
   - Properties: Covers head and neck
   - Value: 25 gold

4. **Steel Helm**
   - AC Bonus: +2
   - Properties: Solid protection, -5% vision range
   - Value: 75 gold

5. **Great Helm**
   - AC Bonus: +3
   - STR requirement: STR 13
   - Properties: Full face protection, -10% vision range
   - Value: 150 gold

**Accessories (Rings, Amulets, Trinkets):**

**Common Accessories:**
1. **Ring of Protection** - +1 AC
2. **Amulet of Health** - +10 max HP
3. **Ring of Strength** - +1 STR
4. **Ring of Dexterity** - +1 DEX
5. **Ring of Constitution** - +1 CON
6. **Cloak of Resistance** - +1 to all saves (future)
7. **Boots of Speed** - +10% movement speed
8. **Gloves of Ogre Power** - +2 STR
9. **Headband of Intellect** - +2 INT
10. **Periapt of Wisdom** - +2 WIS

**Rare Accessories:**
11. **Ring of Regeneration** - Heal 1 HP per 5 seconds
12. **Amulet of Life Drain** - Heal for 20% of damage dealt
13. **Ring of Fire Resistance** - 50% fire damage reduction
14. **Ring of Spell Storing** - Store 1 spell charge (future)
15. **Boots of Elvenkind** - Silent movement
16. **Cloak of Invisibility** - Turn invisible for 10 seconds (60s cooldown)
17. **Belt of Giant Strength** - +4 STR
18. **Ring of Feather Falling** - No fall damage
19. **Amulet of the Devout** - +1 to spell attack rolls and save DCs
20. **Ring of Evasion** - 25% chance to dodge attacks

**Weapon Enchantments:**
Weapons can have magical properties at Uncommon+ rarity:
- **+1/+2/+3 Weapon** - Bonus to attack and damage rolls
- **Flaming** - +1d6 fire damage
- **Frost** - +1d6 cold damage, 30% chance to slow target
- **Shock** - +1d6 lightning damage
- **Vampiric** - Heal for 50% of damage dealt
- **Keen** - Critical hits on 19-20 instead of 20
- **Holy** - +2d6 radiant damage vs undead
- **Venom** - +1d4 poison damage per turn for 3 turns
- **Mighty** - Add STR modifier twice to damage
- **Swift** - -20% attack cooldown
- **Reach** - +0.5 tile range

**Armor Enchantments:**
Armor can have magical properties at Uncommon+ rarity:
- **+1/+2/+3 Armor** - Bonus to AC
- **Resistance (Type)** - 50% damage reduction from one damage type
- **Fortification** - 25% chance to negate critical hits
- **Shadow** - +50% stealth effectiveness
- **Speed** - Ignore armor movement penalty
- **Radiant** - Emit light (10 tile radius), +2 AC vs undead
- **Thorns** - Attackers take 1d6 damage when they hit you
- **Featherweight** - Weight reduced to 0, no STR requirement
- **Absorbing** - Absorb 5 damage per hit (recharges 1/minute)

**Item Identification:**
- Magical items (Uncommon+) appear as "Unidentified [Item Type]" when found
- Must be identified by casting Identify spell or visiting a merchant (costs 10 gold)
- Identified items show their full properties and enchantments
- Common items don't require identification

**Inventory & Equipment:**
- Players and NPCs have inventory (max 20 items) and equipment slots (weapon, armor, shield, helmet, 2 accessories)
- When players or NPCs die, they drop all inventory and equipped items on the map
- Items can be picked up by clicking or walking over them (auto-pickup for nearby items)

**Death & Respawn:**
- Players respawn at a random safe location on the overworld map
- Respawned players start with default equipment (basic sword, cloth armor) and empty inventory
- NPCs respawn after 5 minutes at their spawn point

## NPCs & Combat

**NPC Types:**
- Friendly NPCs: Merchants, quest givers (future), idle wanderers
- Hostile NPCs: Aggressive creatures that attack players on sight
- Neutral NPCs: Ignore players unless attacked

**PvP:** Always enabled. Players can attack other players.

**Combat System:**
- Real-time combat with cooldowns (not turn-based)
- Melee attacks: 1 second cooldown
- Ranged attacks: 1.5 second cooldown (future feature)
- Attack range: Melee 1.5 tiles (48 pixels), Ranged 10 tiles (320 pixels)
- Hit detection: Server checks distance between attacker and target
- Damage calculation: Roll d20 + attack modifier vs AC, then roll weapon damage on hit

## Movement System

**Smooth Continuous Movement:**
- Movement is **not** grid-based; entities move smoothly in continuous 2D space
- Player position is stored as floating-point (x, y) coordinates in pixels
- Movement speed: 200 pixels/second (6.25 tiles/second)
- Movement is fast-paced and fluid, similar to classic Zelda games
- WASD for movement (8-directional: up, down, left, right, diagonals), as well as mouse/trackpad

**Client-Side Prediction:**
- Client immediately moves player on input, predicts position
- Server validates movement (collision detection, speed limits)
- Server sends authoritative position updates
- Client reconciles predicted position with server position (smooth interpolation if mismatch)

## Proximity Systems

**Proximity Chat:**
- Players can hear messages from other players within 15 tiles (480 pixels)
- Chat messages include sender name and appear in chat log
- Server filters messages based on receiver proximity

**Proximity Audio:**
- Sound effects (combat, doors, item pickups) are audible within 20 tiles (640 pixels)
- Volume decreases with distance (linear falloff)

**Vision/Fog of War:**
- Players can see entities and map within 20 tiles (640 pixels) radius
- Server only sends entity updates for entities within player's vision range
- No fog of war rendering in MVP (future feature: true line-of-sight)

# Frontend

## Connection Flow

**Initial UI:**
- Display a connection screen with:
  - Text input for player name (max 16 characters)
  - Settings panel (expandable):
    - Master volume slider (0-100%)
    - Music volume slider (0-100%)
    - SFX volume slider (0-100%)
    - Graphics quality dropdown (future)

**Character Creation UI:**
After entering a name, players proceed to character creation:

1. **Species Selection Screen:**
   - Display 6 species cards in a grid (2x3 or 3x2)
   - Each card shows: species name, portrait, stat bonuses, base HP, movement speed, racial trait
   - Hover shows detailed description
   - Click to select (highlight selected)
   - "Next" button to proceed

2. **Class Selection Screen:**
   - Display 6 class cards in a grid (2x3 or 3x2)
   - Each card shows: class name, icon, primary stats, starting AC, weapon, attack speed, class ability
   - Hover shows detailed description and combined stats with selected species
   - Click to select (highlight selected)
   - "Back" button to return to species selection
   - "Create Character" button to finalize

3. **Character Summary:**
   - Display final character sheet before confirming:
     - Name, species, class
     - All final stats (with bonuses applied)
     - Starting HP, AC, movement speed
     - Starting equipment (weapon, armor)
     - Racial trait and class ability descriptions
   - "Confirm" button to create character
   - "Back" button to return to class selection

**Connection Process:**
1. User enters player name
2. User selects species
3. User selects class
4. User reviews character summary
5. User clicks "Confirm"
6. Frontend initiates WebSocket connection to `ws://localhost:8080/game`
7. On connection success, send `PlayerJoin` message with: name, species, class
8. Server creates character entity with appropriate stats and equipment
9. Server responds with `GameStateSnapshot` (initial full state)
10. Frontend transitions to game UI
11. Begin listening for ongoing `GameStateDelta` messages

**Error Handling:**
- Connection failed: Display error message, allow retry
- Disconnection: Attempt reconnection with exponential backoff, show "Reconnecting..." overlay
- Kicked/banned: Display reason, return to connection screen

## Rendering

**WebGL Rendering:**
- Use WebGL for 2D sprite rendering (via Canvas API with WebGL context)
- Camera follows player, centered on screen
- Viewport renders 25x19 tiles (800x608 pixels) at 1x zoom
- Support zoom levels: 0.5x, 1x, 2x (future)

**Render Pipeline:**
1. Clear canvas
2. Render map tiles (floor layer)
3. Render objects (sorted by y-position for depth)
4. Render entities (players, NPCs) sorted by y-position
5. Render effects (attack animations, particles)
6. Render UI overlay (health bars, chat, inventory)

**Entity Interpolation:**
- Interpolate entity positions between server updates for smooth movement
- Use linear interpolation with 100ms buffer

## Input Handling

**Keyboard Controls:**
- **WASD:** Movement (up, down, left, right, diagonals when multiple keys pressed)
- **Spacebar:** Primary action (attack, interact with nearby objects/NPCs/doors)
- **Enter:** Open chat input
- **I:** Toggle inventory screen
- **M:** Toggle mute (all audio)
- **Esc:** Open menu / close dialogs

**Mouse Controls:**
- **Left click:** Primary action at cursor position (attack if in range, move if far away)
- **Right click:** Context menu for objects/NPCs (future)
- **Mouse wheel:** Zoom in/out (future)

**Input Rate Limiting:**
- Movement input: Send to server at 20 Hz (every 50ms)
- Action input: Send immediately on keypress (respecting server cooldowns)
- Client predicts movement immediately, doesn't wait for server confirmation

## Architecture

**Language & Type Safety:**
- Frontend written in TypeScript for type safety and better developer experience
- Strict TypeScript configuration (`strict: true` in tsconfig.json)
- All game entities, components, and messages strongly typed
- FlatBuffers generates TypeScript types from schema definitions
- No `any` types except where absolutely necessary (external libraries)

**ECS System:**
- Use `bitecs` for entity-component-system architecture
- Components: Position, Velocity, Sprite, Health, Stats, Inventory, etc.
- Systems: RenderSystem, InputSystem, InterpolationSystem, PredictionSystem, etc.
- TypeScript interfaces define component shapes for type checking

**Example TypeScript Component Definitions:**
```typescript
interface Position {
  x: number;
  y: number;
}

interface Velocity {
  dx: number;
  dy: number;
}

interface Health {
  current: number;
  max: number;
}

interface Stats {
  str: number;
  dex: number;
  con: number;
  int: number;
  wis: number;
  cha: number;
}

interface Sprite {
  spriteId: string;
  frame: number;
}

type EntityId = number;

interface Entity {
  id: EntityId;
  position?: Position;
  velocity?: Velocity;
  health?: Health;
  stats?: Stats;
  sprite?: Sprite;
}
```

**Client-Side Prediction:**
- Immediately apply player movement on input
- Store prediction history with sequence numbers
- Reconcile with authoritative server updates:
  - If server position matches prediction: Continue
  - If mismatch: Replay inputs from server position forward

**State Management:**
- Game state stored in ECS world
- UI state (menus, dialogs) stored separately (plain objects or simple state manager)
- All state transitions fully typed

## Audio

The audio system uses Web Audio API for precise control over playback, volume, and crossfading.

**Architecture:**
- AudioManager class (TypeScript singleton) handles all audio playback
- Strongly typed audio manifest interfaces for compile-time validation
- Separate audio contexts for music and sound effects (optional optimization)
- Preload commonly used assets on startup
- Lazy load map-specific music tracks as needed

**Type Definitions:**
```typescript
interface AudioTrack {
  id: string;
  name: string;
  file: string;
  fallback: string;
  loop: boolean;
  loopStart?: number;
  loopEnd?: number | null;
  volume: number;
  description: string;
}

interface AudioManifest {
  music: AudioTrack[];
  ambientSounds: AudioTrack[];
  soundEffects: AudioTrack[];
}

class AudioManager {
  private currentMusic: AudioTrack | null = null;
  private musicVolume: number = 1.0;
  private sfxVolume: number = 1.0;
  private masterVolume: number = 1.0;

  loadManifest(manifest: AudioManifest): Promise<void>;
  playMusic(trackId: string): Promise<void>;
  playSoundEffect(effectId: string, position?: Position): void;
  crossfadeMusic(newTrackId: string, duration: number): Promise<void>;
  setVolume(type: 'master' | 'music' | 'sfx', volume: number): void;
  mute(): void;
  unmute(): void;
}
```

**Background Music:**
- Load music files from audio manifest (OGG primary, MP3 fallback)
- Music loops continuously while on a map
- Support seamless loop points (loopStart/loopEnd in manifest)
- Only one music track plays at a time
- Music persists across chunk loads (no restart)

**Music Transitions:**
- When changing maps with different `backgroundMusic`:
  1. Check if new music is same as current (if yes, continue playing)
  2. If different, start loading new music track
  3. Once loaded, begin 2-second crossfade:
     - Fade out current track (volume 1.0 → 0.0 over 2s)
     - Fade in new track (volume 0.0 → 1.0 over 2s)
     - Use exponential curves for natural-sounding fades
  4. Stop and unload old track after fade completes
- Handle edge case: If music file fails to load, continue current track or play silence

**Ambient Sounds:**
- Optional looping ambient sound layer (separate from music)
- Examples: cave drips, wind, rain, forest birds
- Lower volume than music, provides environmental atmosphere
- Crossfades similar to music (1 second fade)

**Sound Effects:**
- Play sounds based on game events: attack, hit, door open, item pickup, etc.
- Positional audio: Volume and pan based on distance from player
  - Full volume at player position
  - Linear falloff to 0 at 20 tiles distance
  - Stereo panning based on x-axis offset from player
- Multiple instances of same sound can play simultaneously (pooling)
- Sound effects respect cooldowns to prevent spam (max 1 per 50ms per type)

**Volume Controls:**
Three separate volume sliders in settings menu:
- **Master Volume** (0-100%): Multiplied with all audio
- **Music Volume** (0-100%): Affects background music and ambient sounds only
- **SFX Volume** (0-100%): Affects sound effects only
- Settings persist in localStorage

**Audio Loading Strategy:**
- On startup: Load audio manifest and preload common SFX (sword_swing, door_open, item_pickup)
- On map load: Preload map's background music and ambient sound
- Cache loaded audio buffers to avoid re-downloading
- Maximum cache size: 50MB, evict least-recently-used when exceeded

**Mute Functionality:**
- Quick mute toggle (keyboard shortcut 'M')
- Mutes all audio instantly (master volume = 0)
- Restores previous volume settings on unmute
- Mute state persists in localStorage

## Development

**Dependencies:**
Core TypeScript dependencies include:
- `typescript` - TypeScript compiler
- `vite` - Build tool with TypeScript support out of the box
- `bitecs` - ECS library (has TypeScript support)
- `flatbuffers` - Binary serialization (TypeScript codegen)
- `@types/*` - Type definitions for libraries without built-in types

**Dev Server:**
- Use Vite or similar modern dev server with TypeScript support
- Hot module replacement for fast development
- TypeScript compilation with source maps for debugging
- Serve static assets (sprites, sounds, manifests)
- Proxy WebSocket connections to backend (avoid CORS issues)
- Instant TypeScript error feedback in terminal and browser

**Build Process:**
- TypeScript compiled to JavaScript with optimizations
- Production build bundles all TypeScript/CSS into optimized JavaScript
- Tree shaking to remove unused code
- Minification and code splitting
- Assets (images, sounds) copied to dist folder
- Output static site deployable to any web server/CDN

**TypeScript Configuration:**
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve"
  },
  "include": ["src/**/*.ts", "src/**/*.tsx"],
  "exclude": ["node_modules", "dist"]
}
```

# Backend

## Architecture

**Async Runtime:**
- Use Tokio for async runtime
- Accept WebSocket connections on `0.0.0.0:8080/game` (configurable via env vars)
- Each client connection spawns a task to handle incoming messages
- Client messages are queued in a per-client input buffer

**Concurrency Model:**
- Main game loop runs in dedicated task at fixed tick rate
- WebSocket handlers run in separate tasks per client
- Broadcast updates to all clients via channels (tokio mpsc)

## Input Processing

**Input Rate Limiting:**
- Each player has an input slot that stores their latest input
- Movement inputs: Client sends at 20 Hz, server reads once per tick (60 Hz) and uses latest value
- Action inputs: Client sends immediately, server validates cooldowns and processes if ready
- Server ignores inputs that violate cooldowns or physics constraints
- DDoS protection: Clients sending >100 messages/sec are kicked

**Input Validation:**
- Validate all inputs server-side (never trust client)
- Movement: Check speed limits, collision detection, map boundaries
- Actions: Check cooldowns, range, resource costs (future: mana, stamina)
- Combat: Validate attack range, target existence, line-of-sight (future)

## Game Loop

**Tick Rate:** 60 Hz (60 ticks per second = ~16.67ms per tick)

**Main Loop:**
```
loop:
  1. Read latest input from all player input slots
  2. Process player inputs (movement, actions)
  3. Update NPCs (AI decisions, movement, combat)
  4. Update game systems (cooldowns, timers, effects)
  5. Detect collisions and resolve physics
  6. Apply damage, death, respawn logic
  7. Generate delta updates (changed entities/components)
  8. Broadcast delta updates to all clients
  9. Sleep until next tick (maintain 60 Hz)
```

**Delta Updates:**
- Track which entities/components changed this tick
- Only send changed data, not full state
- Include entity ID, component type, new value
- Special message types: EntitySpawned, EntityDespawned

**Broadcast Strategy:**
- Send delta updates to clients based on their loaded chunks and vision range
- Each client receives different updates based on their position
- Use per-client filters to avoid sending irrelevant updates

## State Synchronization

**Initial Connection:**
- Server sends `GameStateSnapshot` with all entities in player's vicinity
- Snapshot includes: map metadata (including `backgroundMusic` and `ambientSound` IDs), nearby chunks, all entities within vision range
- Client immediately starts loading and playing the map's background music

**Ongoing Updates:**
- Server sends `GameStateDelta` messages at 60 Hz (or only when changes occur)
- Deltas include only changed entities/components since last update

**Disconnection Handling:**
- Player entity remains in game for 30 seconds after disconnect (grace period for reconnection)
- After grace period, player is removed and inventory dropped
- Reconnection: Match player by name, restore entity if within grace period

## ECS System

**ECS Framework:** Use `hecs` for entity-component-system architecture

**Components:**
- Position { x: f32, y: f32 }
- Velocity { dx: f32, dy: f32 }
- Health { current: i32, max: i32 }
- Stats { str, dex, con, int, wis, cha: i32 }
- Inventory { items: Vec<Item>, capacity: usize }
- Equipment { weapon, armor, shield: Option<Item> }
- Sprite { sprite_id: String }
- Collider { radius: f32 }
- AI { behavior: AIBehavior, target: Option<EntityId> }
- Player { name: String, connection_id: ConnectionId }
- Cooldowns { attack: Instant, interact: Instant }

**Systems:**
- MovementSystem: Apply velocity to position, check collisions
- AISystem: Update NPC behavior, select targets
- CombatSystem: Process attacks, calculate damage
- RespawnSystem: Handle player/NPC respawn timers
- ChunkSystem: Track entity-chunk associations
- BroadcastSystem: Generate and send delta updates

## AI System (NPC Behavior)

**Hostile NPC AI:**
- Scan for players within 10 tiles
- If player found: Set as target, move toward player
- If in attack range: Attack player
- If player leaves range or dies: Reset to idle state

**Friendly/Neutral NPC AI:**
- Idle: Stand still or wander randomly
- If attacked: Become hostile toward attacker (future)

## Procedural Generation

**Seeded Generation:**
- Use seeded RNG (e.g., `rand_chacha` with seed)
- Seed stored in map metadata (e.g., map ID hash)
- Same seed always generates same map (deterministic)
- Generated once per server lifetime, kept in memory

**Generation Algorithm:**
- Use BSP (Binary Space Partitioning) for dungeon room layout
- Use cellular automata for cave generation
- Use Perlin noise for overworld terrain (future)

## Data Storage

**In-Memory Only:**
- All game state stored in memory (ECS world)
- No database or file persistence (MVP limitation)
- Server restart = full state reset
- Future: Add Redis or PostgreSQL for persistence

**Map Data Loading:**
- Static maps loaded from JSON files in `./rust/data/maps/` directory
- Each map file includes tile data, spawn points, and metadata (including `backgroundMusic` and `ambientSound` IDs)
- Map files reference music/sound IDs; actual audio files are client-side only
- Server sends music IDs to clients in `GameStateSnapshot` and `MapTransition` messages
- Example map file structure:
```json
{
  "id": "overworld_01",
  "name": "Overworld - Starting Area",
  "width": 100,
  "height": 100,
  "backgroundMusic": "overworld_theme",
  "ambientSound": "forest_birds",
  "tileData": [[...], [...]],
  "spawnPoints": [{x: 50, y: 50}, {x: 60, y: 60}],
  "objects": [...]
}
```


# Deployment

## Backend Deployment

**Containerization:**
- Provide a Dockerfile for backend
- Use multi-stage build (build stage + runtime stage)
- Runtime image should be minimal (distroless or alpine)
- Expose port 8080
- Accept configuration via environment variables:
  - `HOST`: Bind address (default: 0.0.0.0)
  - `PORT`: Port number (default: 8080)
  - `LOG_LEVEL`: Logging verbosity (default: info)
  - `MAX_PLAYERS`: Max concurrent players (default: 100)

**Example Dockerfile:**
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/game-server /usr/local/bin/
EXPOSE 8080
CMD ["game-server"]
```

**Container Orchestration:**
- Can be deployed to Docker, Kubernetes, or any container platform
- Single instance only (no horizontal scaling due to in-memory state)
- Future: Add Redis for shared state to enable horizontal scaling

## Frontend Deployment

**Static Build:**
- Run production build: `npm run build` (or equivalent)
- TypeScript compiled to optimized JavaScript
- Output to `./web/dist` directory
- Bundle includes HTML, CSS, compiled JavaScript, and assets (sprites, sounds)
- Type checking performed during build (fails on type errors)
- Deployable to any static hosting: Nginx, Apache, Netlify, Vercel, S3+CloudFront, etc.

**Web Server Configuration:**
- Serve index.html for all routes (SPA routing)
- Enable gzip/brotli compression
- Set cache headers for assets (immutable, 1 year)
- No special server-side logic needed

**Environment Configuration:**
- WebSocket URL configurable via build-time environment variable:
  - `VITE_WS_URL`: WebSocket endpoint (default: ws://localhost:8080/game)

# MVP Scope & Phasing

This is a large project. To ensure we deliver a playable game incrementally, development is broken into phases.

## Phase 1: Core Multiplayer Foundation (MVP)

**Goal:** Playable multiplayer game with basic combat and movement

**Features:**
- ✅ WebSocket connection and messaging (FlatBuffers)
- ✅ Player connection, join, and disconnect
- ✅ Smooth continuous movement (WASD controls)
- ✅ Single static map (loaded from JSON)
- ✅ WebGL rendering with sprites
- ✅ Basic melee combat with cooldowns
- ✅ D&D-inspired stats and damage calculation
- ✅ Simple inventory (pickup, drop items)
- ✅ Player death and respawn
- ✅ Basic hostile NPCs with simple AI
- ✅ Server-authoritative state with client prediction

**Out of Scope for Phase 1:**
- ❌ Chunk loading (use small single map)
- ❌ Procedural generation (static map only)
- ❌ Map transitions / doors
- ❌ Equipment system (basic weapon only)
- ❌ Proximity chat (text chat only)
- ❌ Background music and sound effects (architecture included, assets/playback Phase 4)
- ❌ Friendly NPCs / merchants
- ❌ Complex AI behaviors

## Phase 2: World Exploration

**Add:**
- Chunk-based map loading
- Multiple static maps with doors/transitions
- Basic procedural dungeon generation
- Overworld + dungeon maps
- Fog of war / vision system

## Phase 3: Rich Gameplay

**Add:**
- Full equipment system (armor, shields, accessories)
- Item rarity and stats
- Loot drops and chests
- Friendly NPCs and merchants
- Quest system (basic)
- Improved NPC AI (pathfinding, tactics)

## Phase 4: Social & Polish

**Add:**
- Proximity chat (audio/voice, not just text)
- Background music system (per-map music with crossfading)
- Ambient sound loops (cave drips, forest birds, etc.)
- Sound effects library (combat, interactions, UI)
- Positional audio for all sound effects
- Particle effects and animations (attack effects, spell effects)
- Player customization (sprite selection, color schemes)
- Leaderboards and stats tracking
- Authentication and persistent accounts

## Future Ideas (Post-MVP)

- Magic system and spells
- Crafting and resource gathering
- Player housing
- Guilds / parties
- Boss fights and raids
- Skill trees and character progression
- Mobile client support
