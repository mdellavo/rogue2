export interface Position {
  x: number;
  y: number;
}

export interface Velocity {
  dx: number;
  dy: number;
}

export interface Health {
  current: number;
  max: number;
}

export interface Stats {
  str: number;
  dex: number;
  con: number;
  int: number;
  wis: number;
  cha: number;
}

export interface Sprite {
  spriteId: string;
  frame: number;
}

export type EntityId = number;

export interface Entity {
  id: EntityId;
  position?: Position;
  velocity?: Velocity;
  health?: Health;
  stats?: Stats;
  sprite?: Sprite;
}
