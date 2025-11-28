import { defineComponent, Types } from 'bitecs';

// Position component (f32)
export const Position = defineComponent({
  x: Types.f32,
  y: Types.f32,
});

// Velocity component (f32)
export const Velocity = defineComponent({
  dx: Types.f32,
  dy: Types.f32,
});

// Health component
export const Health = defineComponent({
  current: Types.i16,
  max: Types.i16,
});

// Stats component (D&D stats)
export const Stats = defineComponent({
  str: Types.i8,
  dex: Types.i8,
  con: Types.i8,
  int: Types.i8,
  wis: Types.i8,
  cha: Types.i8,
});

// Sprite component
// Note: spriteId is a string, so we'll store it separately in a Map
// bitecs components can only store numbers
export const Sprite = defineComponent({
  frame: Types.ui8,
});

// ServerEntity component - maps server entity ID to client entity
export const ServerEntity = defineComponent({
  id: Types.ui32,
});

// Player tag component
export const Player = defineComponent();

// Name storage - bitecs can't store strings in components, so we use a Map
export const entityNames = new Map<number, string>();
export const entitySpriteIds = new Map<number, string>();

// Helper to set entity name
export function setEntityName(eid: number, name: string): void {
  entityNames.set(eid, name);
}

// Helper to get entity name
export function getEntityName(eid: number): string | undefined {
  return entityNames.get(eid);
}

// Helper to set sprite ID
export function setEntitySpriteId(eid: number, spriteId: string): void {
  entitySpriteIds.set(eid, spriteId);
}

// Helper to get sprite ID
export function getEntitySpriteId(eid: number): string | undefined {
  return entitySpriteIds.get(eid);
}

// TypeScript interfaces for use outside ECS
export interface PositionData {
  x: number;
  y: number;
}

export interface VelocityData {
  dx: number;
  dy: number;
}
