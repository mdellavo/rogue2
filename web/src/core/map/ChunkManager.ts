/**
 * ChunkManager - Manages map chunks for efficient rendering and memory usage
 *
 * Chunks are 32x32 tiles, loaded in a 3x3 grid around the player.
 * Terrain and feature types are stored in separate indexes to reduce memory.
 */

import type * as FBChunkData from '@generated/game/network/chunk-data';
import type * as FBTerrainIndex from '@generated/game/network/terrain-index';
import type * as FBFeatureIndex from '@generated/game/network/feature-index';

export const CHUNK_SIZE = 32; // 32x32 tiles per chunk
export const TILE_SIZE = 32; // pixels per tile
export const CHUNK_LOAD_RADIUS = 1; // Load 3x3 grid (1 chunk in each direction)

export interface TerrainType {
  id: number;
  terrainType: string;
  walkable: boolean;
  spriteId: string;
}

export interface FeatureType {
  id: number;
  featureType: string;
  blocksMovement: boolean;
  spriteId: string;
}

export interface ChunkFeatureData {
  tileX: number; // 0-31 local position
  tileY: number; // 0-31 local position
  featureId: number;
}

export interface Chunk {
  chunkX: number;
  chunkY: number;
  tiles: Uint32Array; // 1024 tile IDs (32*32)
  features: ChunkFeatureData[];
}

export class ChunkManager {
  private chunks: Map<string, Chunk> = new Map();
  private terrainIndex: Map<number, TerrainType> = new Map();
  private featureIndex: Map<number, FeatureType> = new Map();

  private mapWidthChunks: number = 0;
  private mapHeightChunks: number = 0;

  /**
   * Initialize the chunk manager with terrain and feature indexes
   */
  setIndexes(
    terrainIndex: FBTerrainIndex.TerrainIndex[],
    featureIndex: FBFeatureIndex.FeatureIndex[],
    mapWidthChunks: number,
    mapHeightChunks: number
  ): void {
    // Store terrain index
    this.terrainIndex.clear();
    for (const terrain of terrainIndex) {
      this.terrainIndex.set(terrain.id(), {
        id: terrain.id(),
        terrainType: terrain.terrainType() || '',
        walkable: terrain.walkable(),
        spriteId: terrain.spriteId() || '',
      });
    }

    // Store feature index
    this.featureIndex.clear();
    for (const feature of featureIndex) {
      this.featureIndex.set(feature.id(), {
        id: feature.id(),
        featureType: feature.featureType() || '',
        blocksMovement: feature.blocksMovement(),
        spriteId: feature.spriteId() || '',
      });
    }

    this.mapWidthChunks = mapWidthChunks;
    this.mapHeightChunks = mapHeightChunks;

    console.log(
      `[ChunkManager] Indexes loaded: ${this.terrainIndex.size} terrain types, ${this.featureIndex.size} feature types`
    );
    console.log(`[ChunkManager] Map size: ${mapWidthChunks}x${mapHeightChunks} chunks`);
  }

  /**
   * Load chunks from the server
   */
  loadChunks(chunks: FBChunkData.ChunkData[]): void {
    for (const chunkData of chunks) {
      const chunkX = chunkData.chunkX();
      const chunkY = chunkData.chunkY();
      const key = this.getChunkKey(chunkX, chunkY);

      const tilesArray = chunkData.tilesArray();
      const tiles = tilesArray ? new Uint32Array(tilesArray) : new Uint32Array(CHUNK_SIZE * CHUNK_SIZE);

      const features: ChunkFeatureData[] = [];
      const featuresLength = chunkData.featuresLength();
      for (let i = 0; i < featuresLength; i++) {
        const feature = chunkData.features(i);
        if (feature) {
          features.push({
            tileX: feature.tileX(),
            tileY: feature.tileY(),
            featureId: feature.featureId(),
          });
        }
      }

      this.chunks.set(key, {
        chunkX,
        chunkY,
        tiles,
        features,
      });
    }

    console.log(`[ChunkManager] Loaded ${chunks.length} chunks (total: ${this.chunks.size})`);
  }

  /**
   * Unload chunks from memory
   */
  unloadChunks(chunkCoords: Array<{ x: number; y: number }>): void {
    for (const coord of chunkCoords) {
      const key = this.getChunkKey(coord.x, coord.y);
      this.chunks.delete(key);
    }

    console.log(`[ChunkManager] Unloaded ${chunkCoords.length} chunks (remaining: ${this.chunks.size})`);
  }

  /**
   * Get a chunk by coordinates
   */
  getChunk(chunkX: number, chunkY: number): Chunk | undefined {
    return this.chunks.get(this.getChunkKey(chunkX, chunkY));
  }

  /**
   * Get terrain type by ID
   */
  getTerrainType(id: number): TerrainType | undefined {
    return this.terrainIndex.get(id);
  }

  /**
   * Get feature type by ID
   */
  getFeatureType(id: number): FeatureType | undefined {
    return this.featureIndex.get(id);
  }

  /**
   * Convert world position to chunk coordinates
   */
  worldToChunk(worldX: number, worldY: number): { chunkX: number; chunkY: number } {
    const tileX = Math.floor(worldX / TILE_SIZE);
    const tileY = Math.floor(worldY / TILE_SIZE);
    return {
      chunkX: Math.floor(tileX / CHUNK_SIZE),
      chunkY: Math.floor(tileY / CHUNK_SIZE),
    };
  }

  /**
   * Convert chunk coordinates to world position (top-left corner)
   */
  chunkToWorld(chunkX: number, chunkY: number): { worldX: number; worldY: number } {
    return {
      worldX: chunkX * CHUNK_SIZE * TILE_SIZE,
      worldY: chunkY * CHUNK_SIZE * TILE_SIZE,
    };
  }

  /**
   * Get tile at world position
   */
  getTileAtWorld(worldX: number, worldY: number): number | undefined {
    const tileX = Math.floor(worldX / TILE_SIZE);
    const tileY = Math.floor(worldY / TILE_SIZE);

    const chunkX = Math.floor(tileX / CHUNK_SIZE);
    const chunkY = Math.floor(tileY / CHUNK_SIZE);

    const chunk = this.getChunk(chunkX, chunkY);
    if (!chunk) return undefined;

    const localX = tileX % CHUNK_SIZE;
    const localY = tileY % CHUNK_SIZE;
    const index = localY * CHUNK_SIZE + localX;

    return chunk.tiles[index];
  }

  /**
   * Get all features at world position
   */
  getFeaturesAtWorld(worldX: number, worldY: number): FeatureType[] {
    const tileX = Math.floor(worldX / TILE_SIZE);
    const tileY = Math.floor(worldY / TILE_SIZE);

    const chunkX = Math.floor(tileX / CHUNK_SIZE);
    const chunkY = Math.floor(tileY / CHUNK_SIZE);

    const chunk = this.getChunk(chunkX, chunkY);
    if (!chunk) return [];

    const localX = tileX % CHUNK_SIZE;
    const localY = tileY % CHUNK_SIZE;

    const features: FeatureType[] = [];
    for (const chunkFeature of chunk.features) {
      if (chunkFeature.tileX === localX && chunkFeature.tileY === localY) {
        const featureType = this.getFeatureType(chunkFeature.featureId);
        if (featureType) {
          features.push(featureType);
        }
      }
    }

    return features;
  }

  /**
   * Get chunks needed for a world position (3x3 grid)
   */
  getNeededChunks(worldX: number, worldY: number): Array<{ chunkX: number; chunkY: number }> {
    const { chunkX: centerX, chunkY: centerY } = this.worldToChunk(worldX, worldY);
    const needed: Array<{ chunkX: number; chunkY: number }> = [];

    for (let dy = -CHUNK_LOAD_RADIUS; dy <= CHUNK_LOAD_RADIUS; dy++) {
      for (let dx = -CHUNK_LOAD_RADIUS; dx <= CHUNK_LOAD_RADIUS; dx++) {
        const chunkX = centerX + dx;
        const chunkY = centerY + dy;

        // Check bounds
        if (chunkX >= 0 && chunkY >= 0 && chunkX < this.mapWidthChunks && chunkY < this.mapHeightChunks) {
          needed.push({ chunkX, chunkY });
        }
      }
    }

    return needed;
  }

  /**
   * Get all loaded chunks
   */
  getAllChunks(): Chunk[] {
    return Array.from(this.chunks.values());
  }

  /**
   * Check if a chunk is loaded
   */
  hasChunk(chunkX: number, chunkY: number): boolean {
    return this.chunks.has(this.getChunkKey(chunkX, chunkY));
  }

  /**
   * Clear all chunks
   */
  clear(): void {
    this.chunks.clear();
    console.log('[ChunkManager] Cleared all chunks');
  }

  private getChunkKey(chunkX: number, chunkY: number): string {
    return `${chunkX},${chunkY}`;
  }
}
