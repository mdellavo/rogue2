import { defineQuery, hasComponent, IWorld } from 'bitecs';
import { Renderer } from './Renderer';
import { Position, Sprite, Health, getEntitySpriteId } from '../ecs/components';
import { config } from '@/config';

/**
 * RenderSystem handles drawing all entities to the screen
 */
export class RenderSystem {
  private renderer: Renderer;
  private entityQuery: ReturnType<typeof defineQuery>;

  // Color palette for different entity types (placeholder until we have sprites)
  private readonly colors = {
    player: { r: 0.2, g: 0.6, b: 1.0 }, // Blue
    npc: { r: 1.0, g: 0.3, b: 0.3 }, // Red
    object: { r: 0.8, g: 0.8, b: 0.2 }, // Yellow
    floor: { r: 0.3, g: 0.3, b: 0.35 }, // Dark gray
    wall: { r: 0.5, g: 0.5, b: 0.55 }, // Light gray
  };

  constructor(renderer: Renderer) {
    this.renderer = renderer;

    // Query for all entities with Position and Sprite components
    this.entityQuery = defineQuery([Position, Sprite]);
  }

  /**
   * Render all entities in the world
   */
  public render(world: IWorld, playerEntityId: number | null): void {
    this.renderer.beginFrame();

    // Get all entities to render
    const entities = this.entityQuery(world);

    // Separate entities into layers for proper rendering order
    const sortedEntities = this.sortEntitiesByY(entities);

    // Render entities
    for (const eid of sortedEntities) {
      this.renderEntity(world, eid, playerEntityId);
    }

    this.renderer.endFrame();
  }

  /**
   * Sort entities by Y position for proper depth rendering
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

    // Determine entity color based on type
    const isPlayer = playerEntityId !== null && eid === playerEntityId;
    const spriteId = getEntitySpriteId(eid);

    let color = this.colors.npc;
    if (isPlayer) {
      color = this.colors.player;
    } else if (spriteId?.includes('floor')) {
      color = this.colors.floor;
    } else if (spriteId?.includes('wall')) {
      color = this.colors.wall;
    } else if (spriteId?.includes('door') || spriteId?.includes('chest')) {
      color = this.colors.object;
    }

    // Draw entity as a colored rectangle (placeholder for sprite)
    const size = config.tileSize;
    this.renderer.drawRect(x, y, size, size, color.r, color.g, color.b, 1.0);

    // Draw health bar for entities with health
    if (hasComponent(_world, Health, eid)) {
      this.renderHealthBar(eid, x, y, size);
    }

    // Debug: Draw entity name and ID if enabled
    if (config.debug) {
      // Note: Text rendering would need a separate text rendering system
      // For now, just draw a small indicator above player
      if (isPlayer) {
        this.renderer.drawRect(x + size / 2 - 2, y - 8, 4, 4, 1.0, 1.0, 0.0, 1.0);
      }
    }
  }

  /**
   * Render a health bar above an entity
   */
  private renderHealthBar(eid: number, x: number, y: number, entityWidth: number): void {
    const current = Health.current[eid];
    const max = Health.max[eid];

    if (current === undefined || max === undefined || max === 0) {
      return;
    }

    const healthPercent = current / max;

    // Health bar dimensions
    const barWidth = entityWidth;
    const barHeight = 4;
    const barY = y - 8;

    // Background (dark red)
    this.renderer.drawRect(x, barY, barWidth, barHeight, 0.3, 0.1, 0.1, 0.8);

    // Foreground (health)
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
   * Get health bar color based on health percentage
   */
  private getHealthColor(percent: number): { r: number; g: number; b: number } {
    if (percent > 0.6) {
      return { r: 0.2, g: 0.8, b: 0.2 }; // Green
    } else if (percent > 0.3) {
      return { r: 1.0, g: 0.8, b: 0.0 }; // Yellow
    } else {
      return { r: 1.0, g: 0.2, b: 0.2 }; // Red
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
      // Center camera on player (add half tile size to center on entity)
      const centerX = x + config.tileSize / 2;
      const centerY = y + config.tileSize / 2;
      this.renderer.getCamera().setTarget(centerX, centerY);
    }
  }
}
