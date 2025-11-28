import { ByteBuffer, Builder } from 'flatbuffers';
import * as FBMessage from '@generated/game/network/message';
import * as FBMessageType from '@generated/game/network/message-type';
import * as FBPing from '@generated/game/network/ping';

export enum ConnectionState {
  DISCONNECTED = 'DISCONNECTED',
  CONNECTING = 'CONNECTING',
  CONNECTED = 'CONNECTED',
  RECONNECTING = 'RECONNECTING',
  ERROR = 'ERROR',
}

export interface WebSocketClientEvents {
  onStateChange: (state: ConnectionState) => void;
  onMessage: (message: FBMessage.Message) => void;
  onError: (error: Error) => void;
}

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private url: string;
  private state: ConnectionState = ConnectionState.DISCONNECTED;
  private events: WebSocketClientEvents;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000; // Start with 1 second
  private pingInterval: number | null = null;

  constructor(url: string, events: WebSocketClientEvents) {
    this.url = url;
    this.events = events;
  }

  public connect(): void {
    if (this.state === ConnectionState.CONNECTING || this.state === ConnectionState.CONNECTED) {
      console.warn('WebSocket already connecting or connected');
      return;
    }

    this.setState(ConnectionState.CONNECTING);
    console.log(`🔌 Connecting to ${this.url}...`);

    try {
      this.ws = new WebSocket(this.url);
      this.ws.binaryType = 'arraybuffer';

      this.ws.onopen = this.handleOpen.bind(this);
      this.ws.onmessage = this.handleMessage.bind(this);
      this.ws.onerror = this.handleError.bind(this);
      this.ws.onclose = this.handleClose.bind(this);
    } catch (error) {
      this.handleError(error as Event);
    }
  }

  public disconnect(): void {
    console.log('🔌 Disconnecting...');
    this.stopPingInterval();

    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }

    this.setState(ConnectionState.DISCONNECTED);
  }

  public send(data: Uint8Array): void {
    if (this.state !== ConnectionState.CONNECTED || !this.ws) {
      console.error('Cannot send message: WebSocket not connected');
      return;
    }

    try {
      this.ws.send(data);
    } catch (error) {
      console.error('Error sending message:', error);
      this.events.onError(error as Error);
    }
  }

  public getState(): ConnectionState {
    return this.state;
  }

  private handleOpen(_event: Event): void {
    console.log('✅ WebSocket connected');
    this.reconnectAttempts = 0;
    this.reconnectDelay = 1000;
    this.setState(ConnectionState.CONNECTED);
    this.startPingInterval();
  }

  private handleMessage(event: MessageEvent): void {
    try {
      const buffer = new Uint8Array(event.data);
      const byteBuffer = new ByteBuffer(buffer);
      const message = FBMessage.Message.getRootAsMessage(byteBuffer);

      console.log('📨 Received message, type:', message.payloadType());

      this.events.onMessage(message);
    } catch (error) {
      console.error('Error parsing message:', error);
      this.events.onError(error as Error);
    }
  }

  private handleError(event: Event | Error): void {
    console.error('❌ WebSocket error:', event);
    const error = event instanceof Error ? event : new Error('WebSocket error');
    this.events.onError(error);
    this.setState(ConnectionState.ERROR);
  }

  private handleClose(event: CloseEvent): void {
    console.log('🔌 WebSocket closed:', event.code, event.reason);
    this.stopPingInterval();
    this.ws = null;

    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnect();
    } else {
      console.error('Max reconnection attempts reached');
      this.setState(ConnectionState.DISCONNECTED);
    }
  }

  private reconnect(): void {
    this.reconnectAttempts++;
    const delay = Math.min(this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1), 30000);

    console.log(`🔄 Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
    this.setState(ConnectionState.RECONNECTING);

    setTimeout(() => {
      this.connect();
    }, delay);
  }

  private setState(state: ConnectionState): void {
    if (this.state !== state) {
      this.state = state;
      this.events.onStateChange(state);
    }
  }

  private startPingInterval(): void {
    this.stopPingInterval();

    // Send ping every 5 seconds
    this.pingInterval = window.setInterval(() => {
      this.sendPing();
    }, 5000);
  }

  private stopPingInterval(): void {
    if (this.pingInterval !== null) {
      clearInterval(this.pingInterval);
      this.pingInterval = null;
    }
  }

  private sendPing(): void {
    const builder = new Builder(64);

    // Create Ping message
    const timestamp = BigInt(Date.now());
    FBPing.Ping.startPing(builder);
    FBPing.Ping.addTimestamp(builder, timestamp);
    const pingOffset = FBPing.Ping.endPing(builder);

    // Wrap in Message
    FBMessage.Message.startMessage(builder);
    FBMessage.Message.addPayloadType(builder, FBMessageType.MessageType.Ping);
    FBMessage.Message.addPayload(builder, pingOffset);
    const messageOffset = FBMessage.Message.endMessage(builder);

    builder.finish(messageOffset);

    const buffer = builder.asUint8Array();
    this.send(buffer);
  }
}
