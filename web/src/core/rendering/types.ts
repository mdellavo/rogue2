/**
 * Type definitions for tileset loading and sprite rendering
 */

export interface TilesetMeta {
  tileWidth: number;
  tileHeight: number;
  origin: 'top-left' | 'bottom-left';
  pages: TilesetPage[];
  note?: string;
}

export interface TilesetPage {
  id: string;
  file: string;
  cols: number;
  rows: number;
}

export interface TileData {
  id: string;
  page: string;
  name: string;
  row: number;
  col: number;
  x: number;
  y: number;
  w: number;
  h: number;
  type: string;
  walkable: boolean;
  description: string;
}

export interface TilesetManifest {
  meta: TilesetMeta;
  tiles: TileData[];
}

/**
 * Sprite data with both pixel and UV coordinates
 */
export interface SpriteData {
  // Pixel coordinates in texture
  x: number;
  y: number;
  w: number;
  h: number;

  // UV coordinates (0-1) for WebGL
  u0: number;
  v0: number;
  u1: number;
  v1: number;

  // Metadata
  name: string;
  type: string;
  page: string;
}

/**
 * Complete loaded tileset with texture and sprite lookup
 */
export interface Tileset {
  name: string;
  manifest: TilesetManifest;
  image: HTMLImageElement;
  texture: WebGLTexture | null;
  sprites: Map<string, SpriteData>; // spriteId -> SpriteData
  pages: Map<string, TilesetPage>; // pageId -> TilesetPage
}

/**
 * Loading state for async operations
 */
export enum LoadState {
  NotLoaded = 'not_loaded',
  Loading = 'loading',
  Loaded = 'loaded',
  Error = 'error',
}

/**
 * Loading progress information
 */
export interface LoadProgress {
  state: LoadState;
  progress: number; // 0-1
  error?: string;
}
