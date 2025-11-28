import './ui/styles.css';
import { config } from './config';
import { WebSocketClient, ConnectionState } from './core/network/WebSocketClient';
import type * as FBMessage from '@generated/game/network/message';

console.log('🎮 Game client starting...');
console.log(`WebSocket URL: ${config.wsUrl}`);

// Initialize UI
const app = document.querySelector<HTMLDivElement>('#app')!;
app.innerHTML = `
  <div class="connection-screen">
    <h1>Multiplayer Roguelike</h1>
    <input type="text" id="playerName" placeholder="Enter your name" maxlength="16" />
    <button id="connectBtn">Connect</button>
    <div id="status"></div>
  </div>
`;

// Get UI elements
const playerNameInput = document.querySelector<HTMLInputElement>('#playerName')!;
const connectBtn = document.querySelector<HTMLButtonElement>('#connectBtn')!;
const statusDiv = document.querySelector<HTMLDivElement>('#status')!;

// Create WebSocket client
let wsClient: WebSocketClient | null = null;

// Update status message
function updateStatus(message: string, type: 'info' | 'error' | 'success' = 'info'): void {
  statusDiv.textContent = message;
  statusDiv.className = `status-${type}`;
}

// Handle connection button click
connectBtn.addEventListener('click', () => {
  const playerName = playerNameInput.value.trim();

  if (!playerName) {
    updateStatus('Please enter your name', 'error');
    return;
  }

  if (playerName.length > 16) {
    updateStatus('Name must be 16 characters or less', 'error');
    return;
  }

  // Disable input and button
  playerNameInput.disabled = true;
  connectBtn.disabled = true;
  updateStatus('Connecting...', 'info');

  // Create and connect WebSocket client
  wsClient = new WebSocketClient(config.wsUrl, {
    onStateChange: (state: ConnectionState) => {
      console.log('Connection state:', state);

      switch (state) {
        case ConnectionState.CONNECTING:
          updateStatus('Connecting...', 'info');
          break;
        case ConnectionState.CONNECTED:
          updateStatus('Connected!', 'success');
          break;
        case ConnectionState.RECONNECTING:
          updateStatus('Reconnecting...', 'info');
          break;
        case ConnectionState.DISCONNECTED:
          updateStatus('Disconnected', 'error');
          playerNameInput.disabled = false;
          connectBtn.disabled = false;
          break;
        case ConnectionState.ERROR:
          updateStatus('Connection error', 'error');
          playerNameInput.disabled = false;
          connectBtn.disabled = false;
          break;
      }
    },
    onMessage: (message: FBMessage.Message) => {
      console.log('Received message:', message);
      // TODO: Handle different message types
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
