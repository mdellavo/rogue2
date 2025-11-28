export const config = {
  wsUrl: import.meta.env.VITE_WS_URL || 'ws://localhost:8080/game',
  tickRate: 60,
  inputRate: 20,
  viewportWidth: 800,
  viewportHeight: 608,
  tileSize: 32,
  debug: import.meta.env.DEV,
} as const;
