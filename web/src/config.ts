export const config = {
  wsUrl: import.meta.env.VITE_WS_URL || 'ws://localhost:8080/game',
  assetServerUrl: import.meta.env.VITE_ASSET_SERVER_URL || 'http://localhost:3000',
  defaultTileset: 'dungeon',
  tickRate: 60,
  inputRate: 20,
  viewportWidth: 800,
  viewportHeight: 608,
  tileSize: 32,
  debug: import.meta.env.DEV,
} as const;

/**
 * Get the URL for a tileset image
 */
export function getTilesetImageUrl(name: string): string {
  return `${config.assetServerUrl}/assets/tilesets/tileset_${name}.png`;
}

/**
 * Get the URL for a tileset manifest
 */
export function getTilesetManifestUrl(name: string): string {
  return `${config.assetServerUrl}/manifests/tileset_${name}.json`;
}
