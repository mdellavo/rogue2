import * as FBMessage from '@generated/game/network/message';
import * as FBMessageType from '@generated/game/network/message-type';
import * as FBGameStateSnapshot from '@generated/game/network/game-state-snapshot';
import * as FBGameStateDelta from '@generated/game/network/game-state-delta';
import * as FBPong from '@generated/game/network/pong';
import * as FBMapTransition from '@generated/game/network/map-transition';
import * as FBSystemMessage from '@generated/game/network/system-message';

export interface MessageHandlerCallbacks {
  onSnapshot: (snapshot: FBGameStateSnapshot.GameStateSnapshot) => void;
  onDelta: (delta: FBGameStateDelta.GameStateDelta) => void;
  onMapTransition: (transition: FBMapTransition.MapTransition) => void;
  onSystemMessage: (message: FBSystemMessage.SystemMessage) => void;
  onPong: (pong: FBPong.Pong) => void;
}

export class MessageHandler {
  constructor(private callbacks: MessageHandlerCallbacks) {}

  public handleMessage(message: FBMessage.Message): void {
    const messageType = message.payloadType();

    switch (messageType) {
      case FBMessageType.MessageType.GameStateSnapshot:
        this.handleSnapshot(message);
        break;

      case FBMessageType.MessageType.GameStateDelta:
        this.handleDelta(message);
        break;

      case FBMessageType.MessageType.MapTransition:
        this.handleMapTransition(message);
        break;

      case FBMessageType.MessageType.SystemMessage:
        this.handleSystemMessage(message);
        break;

      case FBMessageType.MessageType.Pong:
        this.handlePong(message);
        break;

      default:
        console.warn('Unhandled message type:', messageType);
    }
  }

  private handleSnapshot(message: FBMessage.Message): void {
    const snapshot = message.payload(new FBGameStateSnapshot.GameStateSnapshot());
    if (!snapshot) {
      console.error('Failed to parse GameStateSnapshot');
      return;
    }

    console.log('📦 Received GameStateSnapshot');
    console.log('  Map:', snapshot.mapName());
    console.log('  Player Entity ID:', snapshot.playerEntityId());
    console.log('  Entities:', snapshot.entitiesLength());

    this.callbacks.onSnapshot(snapshot);
  }

  private handleDelta(message: FBMessage.Message): void {
    const delta = message.payload(new FBGameStateDelta.GameStateDelta());
    if (!delta) {
      console.error('Failed to parse GameStateDelta');
      return;
    }

    // Only log if there are actual changes
    const hasChanges =
      delta.entitiesSpawnedLength() > 0 ||
      delta.entitiesUpdatedLength() > 0 ||
      delta.entitiesDespawnedLength() > 0;

    if (hasChanges) {
      console.log(`📊 Delta #${delta.sequence()}:`, {
        spawned: delta.entitiesSpawnedLength(),
        updated: delta.entitiesUpdatedLength(),
        despawned: delta.entitiesDespawnedLength(),
      });
    }

    this.callbacks.onDelta(delta);
  }

  private handleMapTransition(message: FBMessage.Message): void {
    const transition = message.payload(new FBMapTransition.MapTransition());
    if (!transition) {
      console.error('Failed to parse MapTransition');
      return;
    }

    console.log('🗺️  Map Transition:', transition.mapName());
    this.callbacks.onMapTransition(transition);
  }

  private handleSystemMessage(message: FBMessage.Message): void {
    const systemMessage = message.payload(new FBSystemMessage.SystemMessage());
    if (!systemMessage) {
      console.error('Failed to parse SystemMessage');
      return;
    }

    console.log('💬 System Message:', systemMessage.content());
    this.callbacks.onSystemMessage(systemMessage);
  }

  private handlePong(message: FBMessage.Message): void {
    const pong = message.payload(new FBPong.Pong());
    if (!pong) {
      console.error('Failed to parse Pong');
      return;
    }

    // Silent - pongs are frequent
    this.callbacks.onPong(pong);
  }
}
