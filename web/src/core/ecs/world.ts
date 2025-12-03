import { createWorld, addEntity, addComponent, removeEntity, hasComponent, IWorld } from 'bitecs';
import type * as FBGameStateSnapshot from '@generated/game/network/game-state-snapshot';
import type * as FBEntityData from '@generated/game/network/entity-data';
import type * as FBGameStateDelta from '@generated/game/network/game-state-delta';
import {
  Position,
  Velocity,
  Health,
  Stats,
  Sprite,
  ServerEntity,
  Player,
  setEntityName,
  setEntitySpriteId,
} from './components';

export class GameWorld {
  public world: IWorld;
  private serverToClient: Map<number, number>; // Maps server entity ID to client entity ID
  private clientToServer: Map<number, number>; // Maps client entity ID to server entity ID
  public playerEntityId: number | null = null;

  constructor() {
    this.world = createWorld();
    this.serverToClient = new Map();
    this.clientToServer = new Map();
  }

  /**
   * Initialize the world from a GameStateSnapshot
   */
  public initializeFromSnapshot(
    snapshot: FBGameStateSnapshot.GameStateSnapshot
  ): void {
    console.log('🌍 Initializing ECS world from snapshot...');

    // Clear existing entities
    this.clear();

    // Get player entity ID from snapshot
    const serverPlayerId = snapshot.playerEntityId();

    // Process all entities
    for (let i = 0; i < snapshot.entitiesLength(); i++) {
      const entityData = snapshot.entities(i);
      if (entityData) {
        const clientEid = this.addEntityFromData(entityData);

        // Mark player entity
        if (entityData.id() === serverPlayerId) {
          this.playerEntityId = clientEid;
          addComponent(this.world, Player, clientEid);
          console.log(`👤 Player entity: client=${clientEid}, server=${serverPlayerId}`);
        }
      }
    }

    console.log(`✅ Initialized ${this.serverToClient.size} entities`);
  }

  /**
   * Apply a GameStateDelta to update the world
   */
  public applyDelta(delta: FBGameStateDelta.GameStateDelta): void {
    // Handle spawned entities
    for (let i = 0; i < delta.entitiesSpawnedLength(); i++) {
      const entityData = delta.entitiesSpawned(i);
      if (entityData) {
        const serverId = entityData.id();

        // Check if entity already exists (can happen if entity was in initial snapshot)
        if (this.serverToClient.has(serverId)) {
          console.log(`⚠️ Entity ${serverId} already exists, treating as update instead`);
          this.updateEntityFromData(entityData);
        } else {
          console.log(`✨ Spawning entity ${serverId} (${entityData.name()})`);
          this.addEntityFromData(entityData);
        }
      }
    }

    // Handle updated entities
    for (let i = 0; i < delta.entitiesUpdatedLength(); i++) {
      const entityData = delta.entitiesUpdated(i);
      if (entityData) {
        this.updateEntityFromData(entityData);
      }
    }

    // Handle despawned entities
    for (let i = 0; i < delta.entitiesDespawnedLength(); i++) {
      const serverId = delta.entitiesDespawned(i);
      if (serverId !== null && serverId !== undefined) {
        this.removeEntityByServerId(serverId);
      }
    }
  }

  /**
   * Add a new entity from EntityData
   */
  private addEntityFromData(data: FBEntityData.EntityData): number {
    const serverId = data.id();
    const clientEid = addEntity(this.world);

    // Map server ID to client ID
    this.serverToClient.set(serverId, clientEid);
    this.clientToServer.set(clientEid, serverId);

    // Add ServerEntity component
    addComponent(this.world, ServerEntity, clientEid);
    ServerEntity.id[clientEid] = serverId;

    // Add Position
    const pos = data.position();
    if (pos) {
      addComponent(this.world, Position, clientEid);
      Position.x[clientEid] = pos.x();
      Position.y[clientEid] = pos.y();

      // Always add Velocity component for entities with position (for client-side prediction)
      addComponent(this.world, Velocity, clientEid);
      const vel = data.velocity();
      if (vel) {
        Velocity.dx[clientEid] = vel.x();
        Velocity.dy[clientEid] = vel.y();
      } else {
        Velocity.dx[clientEid] = 0;
        Velocity.dy[clientEid] = 0;
      }
    }

    // Add Health
    const healthCurrent = data.healthCurrent();
    const healthMax = data.healthMax();
    if (healthCurrent !== null && healthMax !== null) {
      addComponent(this.world, Health, clientEid);
      Health.current[clientEid] = healthCurrent;
      Health.max[clientEid] = healthMax;
    }

    // Add Stats
    const stats = data.stats();
    if (stats) {
      addComponent(this.world, Stats, clientEid);
      Stats.str[clientEid] = stats.str();
      Stats.dex[clientEid] = stats.dex();
      Stats.con[clientEid] = stats.con();
      Stats.int[clientEid] = stats.int();
      Stats.wis[clientEid] = stats.wis();
      Stats.cha[clientEid] = stats.cha();
    }

    // Add Sprite
    const spriteId = data.spriteId();
    if (spriteId) {
      addComponent(this.world, Sprite, clientEid);
      Sprite.frame[clientEid] = 0;
      setEntitySpriteId(clientEid, spriteId);
    }

    // Store name
    const name = data.name();
    if (name) {
      setEntityName(clientEid, name);
    }

    return clientEid;
  }

  /**
   * Update an existing entity from EntityData
   */
  private updateEntityFromData(data: FBEntityData.EntityData): void {
    const serverId = data.id();
    const clientEid = this.serverToClient.get(serverId);

    if (clientEid === undefined) {
      console.warn(`Cannot update entity ${serverId}: not found in world`);
      return;
    }

    // Update Position
    const pos = data.position();
    if (pos) {
      const isPlayer = clientEid === this.playerEntityId;
      const oldX = Position.x[clientEid];
      const oldY = Position.y[clientEid];

      if (!hasComponent(this.world, Position, clientEid)) {
        addComponent(this.world, Position, clientEid);
      }
      Position.x[clientEid] = pos.x();
      Position.y[clientEid] = pos.y();

      // Debug: Log position updates for other players
      if (!isPlayer && (Math.abs(pos.x() - oldX) > 0.1 || Math.abs(pos.y() - oldY) > 0.1)) {
        console.log(`📍 Updated entity ${serverId} (${data.name()}): (${oldX.toFixed(1)}, ${oldY.toFixed(1)}) → (${pos.x().toFixed(1)}, ${pos.y().toFixed(1)})`);
      }
    }

    // Update Velocity
    const vel = data.velocity();
    if (vel) {
      if (!hasComponent(this.world, Velocity, clientEid)) {
        addComponent(this.world, Velocity, clientEid);
      }
      Velocity.dx[clientEid] = vel.x();
      Velocity.dy[clientEid] = vel.y();
    }

    // Update Health
    const healthCurrent = data.healthCurrent();
    const healthMax = data.healthMax();
    if (healthCurrent !== null && healthMax !== null) {
      if (!hasComponent(this.world, Health, clientEid)) {
        addComponent(this.world, Health, clientEid);
      }
      Health.current[clientEid] = healthCurrent;
      Health.max[clientEid] = healthMax;
    }

    // Stats typically don't change, but update if present
    const stats = data.stats();
    if (stats) {
      if (!hasComponent(this.world, Stats, clientEid)) {
        addComponent(this.world, Stats, clientEid);
      }
      Stats.str[clientEid] = stats.str();
      Stats.dex[clientEid] = stats.dex();
      Stats.con[clientEid] = stats.con();
      Stats.int[clientEid] = stats.int();
      Stats.wis[clientEid] = stats.wis();
      Stats.cha[clientEid] = stats.cha();
    }
  }

  /**
   * Remove an entity by server ID
   */
  private removeEntityByServerId(serverId: number): void {
    const clientEid = this.serverToClient.get(serverId);
    if (clientEid === undefined) {
      return;
    }

    removeEntity(this.world, clientEid);
    this.serverToClient.delete(serverId);
    this.clientToServer.delete(clientEid);
  }

  /**
   * Get client entity ID from server entity ID
   */
  public getClientEntityId(serverId: number): number | undefined {
    return this.serverToClient.get(serverId);
  }

  /**
   * Get server entity ID from client entity ID
   */
  public getServerEntityId(clientEid: number): number | undefined {
    return this.clientToServer.get(clientEid);
  }

  /**
   * Clear all entities from the world
   */
  public clear(): void {
    // Remove all entities
    this.serverToClient.forEach((clientEid) => {
      removeEntity(this.world, clientEid);
    });

    this.serverToClient.clear();
    this.clientToServer.clear();
    this.playerEntityId = null;
  }

  /**
   * Get the player's client entity ID
   */
  public getPlayerEntityId(): number | null {
    return this.playerEntityId;
  }

  /**
   * Get all client entity IDs
   */
  public getAllEntities(): number[] {
    return Array.from(this.clientToServer.keys());
  }
}
