import { Builder } from 'flatbuffers';
import * as FBMessage from '@generated/game/network/message';
import * as FBMessageType from '@generated/game/network/message-type';
import * as FBPlayerJoin from '@generated/game/network/player-join';
import * as FBPlayerInput from '@generated/game/network/player-input';
import * as FBVec2 from '@generated/game/network/vec2';
import * as FBSpecies from '@generated/game/network/species';
import * as FBCharacterClass from '@generated/game/network/character-class';

export function createPlayerJoinMessage(
  name: string,
  species: FBSpecies.Species,
  characterClass: FBCharacterClass.CharacterClass
): Uint8Array {
  const builder = new Builder(256);

  // Create name string
  const nameOffset = builder.createString(name);

  // Create PlayerJoin
  FBPlayerJoin.PlayerJoin.startPlayerJoin(builder);
  FBPlayerJoin.PlayerJoin.addName(builder, nameOffset);
  FBPlayerJoin.PlayerJoin.addSpecies(builder, species);
  FBPlayerJoin.PlayerJoin.addCharacterClass(builder, characterClass);
  const playerJoinOffset = FBPlayerJoin.PlayerJoin.endPlayerJoin(builder);

  // Wrap in Message
  FBMessage.Message.startMessage(builder);
  FBMessage.Message.addPayloadType(builder, FBMessageType.MessageType.PlayerJoin);
  FBMessage.Message.addPayload(builder, playerJoinOffset);
  const messageOffset = FBMessage.Message.endMessage(builder);

  builder.finish(messageOffset);

  return builder.asUint8Array();
}

export function createPlayerInputMessage(
  sequence: number,
  timestamp: bigint,
  movementX: number,
  movementY: number,
  action: number,
  targetX: number = 0,
  targetY: number = 0
): Uint8Array {
  const builder = new Builder(256);

  // Create Vec2 tables BEFORE the parent PlayerInput table
  // FlatBuffers builds backwards, so create in reverse order of usage
  const targetPosition = FBVec2.Vec2.createVec2(builder, targetX, targetY);
  const movement = FBVec2.Vec2.createVec2(builder, movementX, movementY);

  // Now create PlayerInput table
  FBPlayerInput.PlayerInput.startPlayerInput(builder);
  FBPlayerInput.PlayerInput.addSequence(builder, sequence);
  FBPlayerInput.PlayerInput.addTimestamp(builder, timestamp);
  FBPlayerInput.PlayerInput.addMovement(builder, movement);
  FBPlayerInput.PlayerInput.addAction(builder, action);
  FBPlayerInput.PlayerInput.addTargetPosition(builder, targetPosition);
  const playerInputOffset = FBPlayerInput.PlayerInput.endPlayerInput(builder);

  // Wrap in Message
  FBMessage.Message.startMessage(builder);
  FBMessage.Message.addPayloadType(builder, FBMessageType.MessageType.PlayerInput);
  FBMessage.Message.addPayload(builder, playerInputOffset);
  const messageOffset = FBMessage.Message.endMessage(builder);

  builder.finish(messageOffset);

  return builder.asUint8Array();
}
