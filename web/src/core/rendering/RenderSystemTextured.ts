import { defineQuery, hasComponent, IWorld } from 'bitecs';
import { RendererTextured } from './RendererTextured';
import { TilesetManager } from './TilesetManager';
import { Position, Sprite, Health, getEntitySpriteId } from '../ecs/components';
import { ChunkManager, CHUNK_SIZE, TILE_SIZE } from '../map/ChunkManager';
import { config } from '@/config';

/**
 * RenderSystem with tileset support for sprite-based rendering
 */
export class RenderSystemTextured {
  private renderer: RendererTextured;
  private tilesetManager: TilesetManager;
  private chunkManager: ChunkManager;
  private entityQuery: ReturnType<typeof defineQuery>;

  // Color palette for fallback rendering
  private readonly colors = {
    player: { r: 0.2, g: 0.6, b: 1.0 },
    npc: { r: 1.0, g: 0.3, b: 0.3 },
    otherPlayer: { r: 0.3, g: 1.0, b: 0.3 },
    object: { r: 0.8, g: 0.8, b: 0.2 },
    floor: { r: 0.3, g: 0.3, b: 0.35 },
    wall: { r: 0.5, g: 0.5, b: 0.55 },
  };

  constructor(
    renderer: RendererTextured,
    tilesetManager: TilesetManager,
    chunkManager: ChunkManager
  ) {
    this.renderer = renderer;
    this.tilesetManager = tilesetManager;
    this.chunkManager = chunkManager;
    this.entityQuery = defineQuery([Position, Sprite]);
  }

  /**
   * Render all chunks and entities
   */
  public render(world: IWorld, playerEntityId: number | null): void {
    this.renderer.beginFrame();

    // Render terrain chunks (bottom layer)
    this.renderChunks();

    // Render entities (middle layer)
    const entities = this.entityQuery(world);
    const sortedEntities = this.sortEntitiesByY(entities);

    for (const eid of sortedEntities) {
      this.renderEntity(world, eid, playerEntityId);
    }

    // Render fog of war (top layer)
    if (playerEntityId !== null) {
      const playerX = Position.x[playerEntityId];
      const playerY = Position.y[playerEntityId];
      if (playerX !== undefined && playerY !== undefined) {
        this.renderer.drawFogOfWar(playerX + config.tileSize / 2, playerY + config.tileSize / 2);
      }
    }

    this.renderer.endFrame();
  }

  /**
   * Render all loaded chunks
   */
  private renderChunks(): void {
    const defaultTileset = config.defaultTileset;

    // Only render if tileset is loaded
    if (!this.tilesetManager.isLoaded(defaultTileset)) {
      return;
    }

    const chunks = this.chunkManager.getAllChunks();

    for (const chunk of chunks) {
      // Render terrain tiles
      for (let localY = 0; localY < CHUNK_SIZE; localY++) {
        for (let localX = 0; localX < CHUNK_SIZE; localX++) {
          const tileIndex = localY * CHUNK_SIZE + localX;
          const terrainId = chunk.tiles[tileIndex];
          const terrainType = this.chunkManager.getTerrainType(terrainId);

          if (terrainType) {
            const worldX = (chunk.chunkX * CHUNK_SIZE + localX) * TILE_SIZE;
            const worldY = (chunk.chunkY * CHUNK_SIZE + localY) * TILE_SIZE;

            // Try to render sprite, fall back to colored rect
            const sprite = this.tilesetManager.getSprite(defaultTileset, terrainType.spriteId);
            if (sprite) {
              this.renderer.drawSprite(
                defaultTileset,
                terrainType.spriteId,
                worldX,
                worldY,
                TILE_SIZE,
                TILE_SIZE
              );
            } else {
              // Fallback: color based on terrain type
              const color = this.getTerrainColor(terrainType.terrainType);
              this.renderer.drawRect(worldX, worldY, TILE_SIZE, TILE_SIZE, color.r, color.g, color.b, 1.0);
            }
          }
        }
      }

      // Render features (trees, rocks, etc.) on top of terrain
      for (const feature of chunk.features) {
        const featureType = this.chunkManager.getFeatureType(feature.featureId);
        if (featureType) {
          const worldX = (chunk.chunkX * CHUNK_SIZE + feature.tileX) * TILE_SIZE;
          const worldY = (chunk.chunkY * CHUNK_SIZE + feature.tileY) * TILE_SIZE;

          const sprite = this.tilesetManager.getSprite(defaultTileset, featureType.spriteId);
          if (sprite) {
            this.renderer.drawSprite(
              defaultTileset,
              featureType.spriteId,
              worldX,
              worldY,
              TILE_SIZE,
              TILE_SIZE
            );
          } else {
            // Fallback: draw colored rect for features
            const color = featureType.blocksMovement
              ? { r: 0.4, g: 0.3, b: 0.2 } // Brown for blocking features
              : { r: 0.2, g: 0.6, b: 0.3 }; // Green for non-blocking
            this.renderer.drawRect(worldX, worldY, TILE_SIZE, TILE_SIZE, color.r, color.g, color.b, 1.0);
          }
        }
      }
    }
  }

  /**
   * Get fallback color for terrain types
   */
  private getTerrainColor(terrainType: string): { r: number; g: number; b: number } {
    if (terrainType.includes('grass')) {
      return { r: 0.2, g: 0.6, b: 0.2 };
    } else if (terrainType.includes('forest') || terrainType.includes('tree')) {
      return { r: 0.1, g: 0.4, b: 0.1 };
    } else if (terrainType.includes('desert') || terrainType.includes('sand')) {
      return { r: 0.8, g: 0.7, b: 0.4 };
    } else if (terrainType.includes('water')) {
      return { r: 0.2, g: 0.4, b: 0.8 };
    } else if (terrainType.includes('mountain') || terrainType.includes('rock')) {
      return { r: 0.5, g: 0.5, b: 0.5 };
    } else if (terrainType.includes('snow')) {
      return { r: 0.9, g: 0.9, b: 1.0 };
    } else if (terrainType.includes('beach')) {
      return { r: 0.9, g: 0.8, b: 0.6 };
    }
    return { r: 0.3, g: 0.3, b: 0.35 }; // Default gray
  }

  /**
   * Sort entities by Y position for depth
   */
  private sortEntitiesByY(entities: number[]): number[] {
    return entities.slice().sort((a, b) => {
      const yA = Position.y[a] || 0;
      const yB = Position.y[b] || 0;
      return yA - yB;
    });
  }

  /**
   * Render a single entity
   */
  private renderEntity(_world: IWorld, eid: number, playerEntityId: number | null): void {
    const x = Position.x[eid];
    const y = Position.y[eid];
    const spriteId = getEntitySpriteId(eid);
    const isPlayer = playerEntityId !== null && eid === playerEntityId;

    const defaultTileset = config.defaultTileset;
    const size = config.tileSize;

    // Try to render sprite
    const sprite = this.tilesetManager.getSprite(defaultTileset, spriteId || 'unknown');
    if (sprite) {
      this.renderer.drawSprite(defaultTileset, spriteId || 'unknown', x, y, size, size);
    } else {
      // Fallback to colored rectangle
      let color = this.colors.npc;
      if (isPlayer) {
        color = this.colors.player;
      } else if (
        spriteId?.includes('human') ||
        spriteId?.includes('elf') ||
        spriteId?.includes('dwarf') ||
        spriteId?.includes('fighter') ||
        spriteId?.includes('rogue') ||
        spriteId?.includes('wizard')
      ) {
        color = this.colors.otherPlayer;
      } else if (spriteId?.includes('floor')) {
        color = this.colors.floor;
      } else if (spriteId?.includes('wall')) {
        color = this.colors.wall;
      } else if (spriteId?.includes('door') || spriteId?.includes('chest')) {
        color = this.colors.object;
      }

      this.renderer.drawRect(x, y, size, size, color.r, color.g, color.b, 1.0);
    }

    // Draw health bar
    if (hasComponent(_world, Health, eid)) {
      this.renderHealthBar(eid, x, y, size);
    }

    // Debug indicator for player
    if (config.debug && isPlayer) {
      this.renderer.drawRect(x + size / 2 - 2, y - 8, 4, 4, 1.0, 1.0, 0.0, 1.0);
    }
  }

  /**
   * Render health bar
   */
  private renderHealthBar(eid: number, x: number, y: number, entityWidth: number): void {
    const current = Health.current[eid];
    const max = Health.max[eid];

    if (current === undefined || max === undefined || max === 0) {
      return;
    }

    const healthPercent = current / max;
    const barWidth = entityWidth;
    const barHeight = 4;
    const barY = y - 8;

    // Background
    this.renderer.drawRect(x, barY, barWidth, barHeight, 0.3, 0.1, 0.1, 0.8);

    // Foreground
    const healthColor = this.getHealthColor(healthPercent);
    this.renderer.drawRect(
      x,
      barY,
      barWidth * healthPercent,
      barHeight,
      healthColor.r,
      healthColor.g,
      healthColor.b,
      0.9
    );
  }

  /**
   * Get health bar color
   */
  private getHealthColor(percent: number): { r: number; g: number; b: number } {
    if (percent > 0.6) {
      return { r: 0.2, g: 0.8, b: 0.2 };
    } else if (percent > 0.3) {
      return { r: 1.0, g: 0.8, b: 0.0 };
    } else {
      return { r: 1.0, g: 0.2, b: 0.2 };
    }
  }

  /**
   * Update camera to follow player
   */
  public updateCamera(playerEntityId: number | null): void {
    if (playerEntityId === null) {
      return;
    }

    const x = Position.x[playerEntityId];
    const y = Position.y[playerEntityId];

    if (x !== undefined && y !== undefined) {
      const centerX = x + config.tileSize / 2;
      const centerY = y + config.tileSize / 2;
      this.renderer.getCamera().setTarget(centerX, centerY);
    }
  }
}
