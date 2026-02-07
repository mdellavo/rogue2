/**
 * TilesetManager - Manages loading and caching of tilesets
 *
 * Responsibilities:
 * - Load tileset manifests and images from URLs
 * - Create WebGL textures from loaded images
 * - Provide sprite lookup by ID
 * - Track loading state and progress
 * - Handle errors with retries
 */

import { getTilesetImageUrl, getTilesetManifestUrl } from '@/config';
import {
  type Tileset,
  type TilesetManifest,
  type SpriteData,
  LoadState,
  type LoadProgress,
} from './types';

const MAX_RETRIES = 3;
const RETRY_DELAY_MS = 1000;

export class TilesetManager {
  private tilesets: Map<string, Tileset> = new Map();
  private loadStates: Map<string, LoadProgress> = new Map();
  private gl: WebGLRenderingContext | null = null;

  /**
   * Initialize with WebGL context for texture creation
   */
  setGLContext(gl: WebGLRenderingContext): void {
    this.gl = gl;
  }

  /**
   * Load a tileset by name
   */
  async loadTileset(name: string): Promise<Tileset> {
    // Check if already loaded
    const existing = this.tilesets.get(name);
    if (existing) {
      return existing;
    }

    // Check if already loading
    const loadState = this.loadStates.get(name);
    if (loadState && loadState.state === LoadState.Loading) {
      // Wait for existing load to complete
      return this.waitForLoad(name);
    }

    // Start loading
    this.setLoadState(name, LoadState.Loading, 0);

    try {
      // Load manifest and image in parallel
      const [manifest, image] = await Promise.all([
        this.loadManifest(name),
        this.loadImage(name),
      ]);

      this.setLoadState(name, LoadState.Loading, 0.8);

      // Create WebGL texture
      const texture = this.gl ? this.createTexture(image) : null;

      this.setLoadState(name, LoadState.Loading, 0.9);

      // Build sprite lookup map
      const sprites = this.buildSpriteMap(manifest, image.width, image.height);
      const pages = this.buildPageMap(manifest);

      const tileset: Tileset = {
        name,
        manifest,
        image,
        texture,
        sprites,
        pages,
      };

      this.tilesets.set(name, tileset);
      this.setLoadState(name, LoadState.Loaded, 1.0);

      console.log(`[TilesetManager] Loaded tileset "${name}":`, {
        sprites: sprites.size,
        pages: pages.size,
        imageSize: `${image.width}x${image.height}`,
      });

      return tileset;
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error);
      this.setLoadState(name, LoadState.Error, 0, errorMsg);
      console.error(`[TilesetManager] Failed to load tileset "${name}":`, error);
      throw error;
    }
  }

  /**
   * Load tileset manifest JSON
   */
  private async loadManifest(name: string, retries = 0): Promise<TilesetManifest> {
    const url = getTilesetManifestUrl(name);

    try {
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      const manifest = await response.json();

      // Validate manifest structure
      if (!manifest.meta || !manifest.tiles || !Array.isArray(manifest.tiles)) {
        throw new Error('Invalid manifest format: missing meta or tiles');
      }

      return manifest as TilesetManifest;
    } catch (error) {
      if (retries < MAX_RETRIES) {
        console.warn(
          `[TilesetManager] Manifest load failed, retrying (${retries + 1}/${MAX_RETRIES})...`
        );
        await this.sleep(RETRY_DELAY_MS * (retries + 1));
        return this.loadManifest(name, retries + 1);
      }
      throw new Error(`Failed to load manifest: ${error}`);
    }
  }

  /**
   * Load tileset image
   */
  private async loadImage(name: string, retries = 0): Promise<HTMLImageElement> {
    const url = getTilesetImageUrl(name);

    return new Promise((resolve, reject) => {
      const img = new Image();

      img.onload = () => {
        console.log(`[TilesetManager] Image loaded: ${url} (${img.width}x${img.height})`);
        resolve(img);
      };

      img.onerror = async () => {
        if (retries < MAX_RETRIES) {
          console.warn(
            `[TilesetManager] Image load failed, retrying (${retries + 1}/${MAX_RETRIES})...`
          );
          await this.sleep(RETRY_DELAY_MS * (retries + 1));
          try {
            const retryImg = await this.loadImage(name, retries + 1);
            resolve(retryImg);
          } catch (error) {
            reject(error);
          }
        } else {
          reject(new Error(`Failed to load image: ${url}`));
        }
      };

      img.crossOrigin = 'anonymous'; // Enable CORS
      img.src = url;
    });
  }

  /**
   * Create WebGL texture from image
   */
  private createTexture(image: HTMLImageElement): WebGLTexture | null {
    if (!this.gl) {
      console.warn('[TilesetManager] No WebGL context, cannot create texture');
      return null;
    }

    const gl = this.gl;
    const texture = gl.createTexture();
    if (!texture) {
      console.error('[TilesetManager] Failed to create texture');
      return null;
    }

    gl.bindTexture(gl.TEXTURE_2D, texture);

    // Set texture parameters
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST); // Pixel-perfect scaling

    // Upload image to GPU
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);

    // Check for power-of-2 dimensions (required for mipmaps)
    const isPowerOf2 = (value: number) => (value & (value - 1)) === 0;
    if (isPowerOf2(image.width) && isPowerOf2(image.height)) {
      gl.generateMipmap(gl.TEXTURE_2D);
    }

    console.log('[TilesetManager] Created WebGL texture');
    return texture;
  }

  /**
   * Build sprite lookup map with UV coordinates
   */
  private buildSpriteMap(
    manifest: TilesetManifest,
    imageWidth: number,
    imageHeight: number
  ): Map<string, SpriteData> {
    const sprites = new Map<string, SpriteData>();

    for (const tile of manifest.tiles) {
      // Calculate UV coordinates (0-1 range)
      const u0 = tile.x / imageWidth;
      const v0 = tile.y / imageHeight;
      const u1 = (tile.x + tile.w) / imageWidth;
      const v1 = (tile.y + tile.h) / imageHeight;

      const spriteData: SpriteData = {
        x: tile.x,
        y: tile.y,
        w: tile.w,
        h: tile.h,
        u0,
        v0,
        u1,
        v1,
        name: tile.name,
        type: tile.type,
        page: tile.page,
      };

      sprites.set(tile.id, spriteData);
    }

    return sprites;
  }

  /**
   * Build page lookup map
   */
  private buildPageMap(manifest: TilesetManifest): Map<string, any> {
    const pages = new Map();
    for (const page of manifest.meta.pages) {
      pages.set(page.id, page);
    }
    return pages;
  }

  /**
   * Get sprite data by ID
   */
  getSprite(tilesetName: string, spriteId: string): SpriteData | undefined {
    const tileset = this.tilesets.get(tilesetName);
    return tileset?.sprites.get(spriteId);
  }

  /**
   * Get WebGL texture
   */
  getTexture(tilesetName: string): WebGLTexture | null {
    return this.tilesets.get(tilesetName)?.texture || null;
  }

  /**
   * Check if tileset is loaded
   */
  isLoaded(tilesetName: string): boolean {
    return this.tilesets.has(tilesetName);
  }

  /**
   * Get loading progress (0-1)
   */
  getProgress(tilesetName: string): LoadProgress {
    return (
      this.loadStates.get(tilesetName) || {
        state: LoadState.NotLoaded,
        progress: 0,
      }
    );
  }

  /**
   * Get all loaded tileset names
   */
  getLoadedTilesets(): string[] {
    return Array.from(this.tilesets.keys());
  }

  /**
   * Unload a tileset to free memory
   */
  unloadTileset(name: string): void {
    const tileset = this.tilesets.get(name);
    if (tileset && tileset.texture && this.gl) {
      this.gl.deleteTexture(tileset.texture);
    }
    this.tilesets.delete(name);
    this.loadStates.delete(name);
    console.log(`[TilesetManager] Unloaded tileset "${name}"`);
  }

  /**
   * Clear all tilesets
   */
  clear(): void {
    for (const name of this.tilesets.keys()) {
      this.unloadTileset(name);
    }
  }

  // Helper methods

  private setLoadState(
    name: string,
    state: LoadState,
    progress: number,
    error?: string
  ): void {
    this.loadStates.set(name, { state, progress, error });
  }

  private async waitForLoad(name: string): Promise<Tileset> {
    // Poll until loaded or error
    return new Promise((resolve, reject) => {
      const checkInterval = setInterval(() => {
        const loadState = this.loadStates.get(name);
        if (loadState?.state === LoadState.Loaded) {
          clearInterval(checkInterval);
          const tileset = this.tilesets.get(name);
          if (tileset) {
            resolve(tileset);
          } else {
            reject(new Error(`Tileset "${name}" loaded but not found`));
          }
        } else if (loadState?.state === LoadState.Error) {
          clearInterval(checkInterval);
          reject(new Error(loadState.error || 'Unknown error'));
        }
      }, 100);
    });
  }

  private sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
}
