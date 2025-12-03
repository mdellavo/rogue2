import './ui/styles.css';
import { config } from './config';
import { WebSocketClient, ConnectionState } from './core/network/WebSocketClient';
import { MessageHandler } from './core/network/MessageHandler';
import { createPlayerJoinMessage } from './core/network/messages';
import { GameWorld } from './core/ecs/world';
import { Camera, Renderer, RenderSystem } from './core/rendering';
import { InputManager, InputSystem } from './core/input';
import { ChunkManager } from './core/map/ChunkManager';
import * as FBSpecies from '@generated/game/network/species';
import * as FBCharacterClass from '@generated/game/network/character-class';
import type * as FBGameStateSnapshot from '@generated/game/network/game-state-snapshot';
import type * as FBGameStateDelta from '@generated/game/network/game-state-delta';
import type * as FBMapTransition from '@generated/game/network/map-transition';
import type * as FBSystemMessage from '@generated/game/network/system-message';
import type * as FBPong from '@generated/game/network/pong';
import type * as FBChunksLoaded from '@generated/game/network/chunks-loaded';
import type * as FBChunksUnloaded from '@generated/game/network/chunks-unloaded';

console.log('🎮 Game client starting...');
console.log(`WebSocket URL: ${config.wsUrl}`);

// Initialize UI
const app = document.querySelector<HTMLDivElement>('#app')!;
app.innerHTML = `
  <div class="connection-screen">
    <h1>Multiplayer Roguelike</h1>

    <div class="character-creation">
      <div class="form-group">
        <label for="playerName">Character Name:</label>
        <input type="text" id="playerName" placeholder="Enter your name" maxlength="16" />
      </div>

      <div class="form-group">
        <label for="species">Species:</label>
        <select id="species">
          <option value="${FBSpecies.Species.Human}">Human</option>
          <option value="${FBSpecies.Species.Elf}">Elf</option>
          <option value="${FBSpecies.Species.Dwarf}">Dwarf</option>
          <option value="${FBSpecies.Species.Halfling}">Halfling</option>
          <option value="${FBSpecies.Species.HalfOrc}">Half-Orc</option>
          <option value="${FBSpecies.Species.Gnome}">Gnome</option>
        </select>
      </div>

      <div class="form-group">
        <label for="characterClass">Class:</label>
        <select id="characterClass">
          <option value="${FBCharacterClass.CharacterClass.Fighter}">Fighter</option>
          <option value="${FBCharacterClass.CharacterClass.Rogue}">Rogue</option>
          <option value="${FBCharacterClass.CharacterClass.Cleric}">Cleric</option>
          <option value="${FBCharacterClass.CharacterClass.Wizard}">Wizard</option>
          <option value="${FBCharacterClass.CharacterClass.Ranger}">Ranger</option>
          <option value="${FBCharacterClass.CharacterClass.Barbarian}">Barbarian</option>
        </select>
      </div>

      <button id="connectBtn">Join Game</button>
      <div id="status"></div>
    </div>

    <div id="game-container" style="display: none;">
      <div id="game-info"></div>
      <canvas id="game-canvas"></canvas>
    </div>
  </div>
`;

// Get UI elements
const playerNameInput = document.querySelector<HTMLInputElement>('#playerName')!;
const speciesSelect = document.querySelector<HTMLSelectElement>('#species')!;
const classSelect = document.querySelector<HTMLSelectElement>('#characterClass')!;
const connectBtn = document.querySelector<HTMLButtonElement>('#connectBtn')!;
const statusDiv = document.querySelector<HTMLDivElement>('#status')!;
const characterCreationDiv = document.querySelector<HTMLDivElement>('.character-creation')!;
const gameContainerDiv = document.querySelector<HTMLDivElement>('#game-container')!;
const gameInfoDiv = document.querySelector<HTMLDivElement>('#game-info')!;
const canvas = document.querySelector<HTMLCanvasElement>('#game-canvas')!;

// Create game world and WebSocket client
const gameWorld = new GameWorld();
const chunkManager = new ChunkManager();
let wsClient: WebSocketClient | null = null;
let messageHandler: MessageHandler | null = null;
let playerEntityId: number | null = null;

// Rendering system
let camera: Camera | null = null;
let renderer: Renderer | null = null;
let renderSystem: RenderSystem | null = null;
let gameLoopRunning: boolean = false;

// Input system
let inputManager: InputManager | null = null;
let inputSystem: InputSystem | null = null;

// Update status message
function updateStatus(message: string, type: 'info' | 'error' | 'success' = 'info'): void {
  statusDiv.textContent = message;
  statusDiv.className = `status-${type}`;
}

// Show game UI and hide character creation
function showGame(): void {
  characterCreationDiv.style.display = 'none';
  gameContainerDiv.style.display = 'block';
}

// Handle GameStateSnapshot
function handleGameStateSnapshot(snapshot: FBGameStateSnapshot.GameStateSnapshot): void {
  console.log('🎮 Initializing game from snapshot...');

  // Store player entity ID
  playerEntityId = snapshot.playerEntityId();

  // Initialize chunk system with indexes
  const terrainIndex: any[] = [];
  for (let i = 0; i < snapshot.terrainIndexLength(); i++) {
    const terrain = snapshot.terrainIndex(i);
    if (terrain) terrainIndex.push(terrain);
  }

  const featureIndex: any[] = [];
  for (let i = 0; i < snapshot.featureIndexLength(); i++) {
    const feature = snapshot.featureIndex(i);
    if (feature) featureIndex.push(feature);
  }

  chunkManager.setIndexes(
    terrainIndex,
    featureIndex,
    snapshot.mapWidthChunks(),
    snapshot.mapHeightChunks()
  );

  // Load initial chunks
  const chunks: any[] = [];
  for (let i = 0; i < snapshot.chunksLength(); i++) {
    const chunk = snapshot.chunks(i);
    if (chunk) chunks.push(chunk);
  }
  chunkManager.loadChunks(chunks);

  // Display game info
  gameInfoDiv.innerHTML = `
    <div class="info-panel">
      <h3>${snapshot.mapName()}</h3>
      <p>Player Entity ID: ${playerEntityId}</p>
      <p>Entities in view: ${snapshot.entitiesLength()}</p>
      <p>Chunks loaded: ${chunks.length}</p>
      <p>Music: ${snapshot.backgroundMusic()}</p>
      <p>Ambient: ${snapshot.ambientSound()}</p>
    </div>
  `;

  // Log all entities
  console.log(`📋 Received ${snapshot.entitiesLength()} entities:`);
  for (let i = 0; i < snapshot.entitiesLength(); i++) {
    const entity = snapshot.entities(i);
    if (entity) {
      const pos = entity.position();
      console.log(`  Entity #${entity.id()}: ${entity.name()} at (${pos?.x()}, ${pos?.y()})`);
      console.log(`    Health: ${entity.healthCurrent()}/${entity.healthMax()}`);
      const stats = entity.stats();
      if (stats) {
        console.log(`    Stats: STR ${stats.str()} DEX ${stats.dex()} CON ${stats.con()}`);
      }
    }
  }

  // Initialize ECS world with snapshot data
  gameWorld.initializeFromSnapshot(snapshot);
  console.log(`✅ ECS world initialized with ${gameWorld.getAllEntities().length} entities`);
  console.log(`👤 Player client entity ID: ${gameWorld.getPlayerEntityId()}`);

  // Initialize renderer
  if (!camera) {
    camera = new Camera();
    renderer = new Renderer(canvas, camera);
    renderSystem = new RenderSystem(renderer);
    console.log('🎨 Renderer initialized');
  }

  // Initialize input system
  if (!inputManager && wsClient) {
    inputManager = new InputManager();
    inputSystem = new InputSystem(inputManager, wsClient, camera, canvas);
    inputSystem.enable();
    console.log('🎮 Input system initialized');
  }

  // Start game loop
  if (!gameLoopRunning) {
    gameLoopRunning = true;
    startGameLoop();
    console.log('🔄 Game loop started');
  }

  // Show game UI
  showGame();
  updateStatus('Game started!', 'success');
}

// Handle GameStateDelta
function handleGameStateDelta(delta: FBGameStateDelta.GameStateDelta): void {
  // Apply delta updates to ECS world
  gameWorld.applyDelta(delta);

  // Log significant changes
  if (
    delta.entitiesSpawnedLength() > 0 ||
    delta.entitiesUpdatedLength() > 0 ||
    delta.entitiesDespawnedLength() > 0
  ) {
    console.log(
      `🔄 Delta #${delta.sequence()}: ` +
        `spawned=${delta.entitiesSpawnedLength()} ` +
        `updated=${delta.entitiesUpdatedLength()} ` +
        `despawned=${delta.entitiesDespawnedLength()}`
    );
  }
}

// Handle MapTransition
function handleMapTransition(transition: FBMapTransition.MapTransition): void {
  console.log('🗺️  Transitioning to new map:', transition.mapName());

  // Clear current world state
  gameWorld.clear();
  console.log('🧹 Cleared ECS world for map transition');

  // TODO: Clear renderer state
  // Server will send a new snapshot with the new map state

  updateStatus(`Entering ${transition.mapName()}...`, 'info');
}

// Handle SystemMessage
function handleSystemMessage(message: FBSystemMessage.SystemMessage): void {
  const msg = message.content();
  console.log('💬', msg);
  // TODO: Display in chat UI
}

// Handle Pong
function handlePong(_pong: FBPong.Pong): void {
  // Pong received, connection is alive
}

// Handle ChunksLoaded
function handleChunksLoaded(chunksLoaded: FBChunksLoaded.ChunksLoaded): void {
  const chunks: any[] = [];
  for (let i = 0; i < chunksLoaded.chunksLength(); i++) {
    const chunk = chunksLoaded.chunks(i);
    if (chunk) chunks.push(chunk);
  }
  chunkManager.loadChunks(chunks);
}

// Handle ChunksUnloaded
function handleChunksUnloaded(chunksUnloaded: FBChunksUnloaded.ChunksUnloaded): void {
  const coords: Array<{ x: number; y: number }> = [];
  for (let i = 0; i < chunksUnloaded.chunkCoordsLength(); i++) {
    const coord = chunksUnloaded.chunkCoords(i);
    if (coord) {
      coords.push({ x: coord.x(), y: coord.y() });
    }
  }
  chunkManager.unloadChunks(coords);
}

// Game loop
let lastFrameTime = performance.now();

function startGameLoop(): void {
  function gameLoop(currentTime: number): void {
    if (!gameLoopRunning) {
      return;
    }

    // Calculate delta time in seconds
    const deltaTime = (currentTime - lastFrameTime) / 1000;
    lastFrameTime = currentTime;

    // Update input system (handles client-side prediction and sends input to server)
    if (inputSystem) {
      inputSystem.update(deltaTime, gameWorld);
    }

    // Update camera to follow player
    if (camera && renderSystem) {
      camera.update(deltaTime);
      renderSystem.updateCamera(gameWorld.getPlayerEntityId());
    }

    // Render the frame
    if (renderSystem) {
      renderSystem.render(gameWorld.world, gameWorld.getPlayerEntityId());
    }

    // Request next frame
    requestAnimationFrame(gameLoop);
  }

  // Start the loop
  lastFrameTime = performance.now();
  requestAnimationFrame(gameLoop);
}

// Handle connection button click
connectBtn.addEventListener('click', () => {
  const playerName = playerNameInput.value.trim();
  const species = parseInt(speciesSelect.value) as FBSpecies.Species;
  const characterClass = parseInt(classSelect.value) as FBCharacterClass.CharacterClass;

  // Validate input
  if (!playerName) {
    updateStatus('Please enter your name', 'error');
    return;
  }

  if (playerName.length > 16) {
    updateStatus('Name must be 16 characters or less', 'error');
    return;
  }

  // Disable inputs
  playerNameInput.disabled = true;
  speciesSelect.disabled = true;
  classSelect.disabled = true;
  connectBtn.disabled = true;
  updateStatus('Connecting...', 'info');

  // Create message handler
  messageHandler = new MessageHandler({
    onSnapshot: handleGameStateSnapshot,
    onDelta: handleGameStateDelta,
    onMapTransition: handleMapTransition,
    onSystemMessage: handleSystemMessage,
    onPong: handlePong,
    onChunksLoaded: handleChunksLoaded,
    onChunksUnloaded: handleChunksUnloaded,
  });

  // Create WebSocket client
  wsClient = new WebSocketClient(config.wsUrl, {
    onStateChange: (state: ConnectionState) => {
      console.log('Connection state:', state);

      switch (state) {
        case ConnectionState.CONNECTING:
          updateStatus('Connecting...', 'info');
          break;

        case ConnectionState.CONNECTED:
          updateStatus('Connected! Joining game...', 'success');

          // Send PlayerJoin message
          const joinMessage = createPlayerJoinMessage(playerName, species, characterClass);
          wsClient!.send(joinMessage);
          console.log(`📤 Sent PlayerJoin: ${playerName} (${species}, ${characterClass})`);
          break;

        case ConnectionState.RECONNECTING:
          updateStatus('Reconnecting...', 'info');
          break;

        case ConnectionState.DISCONNECTED:
          updateStatus('Disconnected', 'error');
          // Re-enable inputs
          playerNameInput.disabled = false;
          speciesSelect.disabled = false;
          classSelect.disabled = false;
          connectBtn.disabled = false;
          break;

        case ConnectionState.ERROR:
          updateStatus('Connection error', 'error');
          // Re-enable inputs
          playerNameInput.disabled = false;
          speciesSelect.disabled = false;
          classSelect.disabled = false;
          connectBtn.disabled = false;
          break;
      }
    },

    onMessage: (message) => {
      if (messageHandler) {
        messageHandler.handleMessage(message);
      }
    },

    onError: (error: Error) => {
      console.error('WebSocket error:', error);
      updateStatus(`Error: ${error.message}`, 'error');
    },
  });

  wsClient.connect();
});

// Handle Enter key in name input
playerNameInput.addEventListener('keypress', (e) => {
  if (e.key === 'Enter') {
    connectBtn.click();
  }
});
