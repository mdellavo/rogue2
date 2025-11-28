import { InputManager } from './InputManager';
import { GameWorld } from '../ecs/world';
import { Position, Velocity } from '../ecs/components';
import { Camera } from '../rendering/Camera';
import { createPlayerInputMessage } from '../network/messages';
import { WebSocketClient } from '../network/WebSocketClient';
import { config } from '@/config';

/**
 * InputSystem handles sending input to the server and client-side prediction
 */
export class InputSystem {
  private inputManager: InputManager;
  private wsClient: WebSocketClient;
  private camera: Camera;
  private canvas: HTMLCanvasElement;

  // Input sequence tracking
  private inputSequence: number = 0;

  // Rate limiting
  private lastInputSendTime: number = 0;
  private readonly inputSendInterval: number = 1000 / config.inputRate; // 50ms for 20 Hz

  // Movement speed (pixels per second)
  private readonly movementSpeed: number = 200;

  constructor(
    inputManager: InputManager,
    wsClient: WebSocketClient,
    camera: Camera,
    canvas: HTMLCanvasElement
  ) {
    this.inputManager = inputManager;
    this.wsClient = wsClient;
    this.camera = camera;
    this.canvas = canvas;
  }

  /**
   * Update the input system
   * Should be called every frame
   */
  public update(deltaTime: number, gameWorld: GameWorld): void {
    const playerEntityId = gameWorld.getPlayerEntityId();
    if (playerEntityId === null || !this.inputManager.isEnabled()) {
      return;
    }

    // Get movement input
    const movement = this.inputManager.getMovementVector();

    // Get action input
    const action = this.inputManager.getAction();

    // Get mouse click for targeting
    const mouseClick = this.inputManager.consumeMouseClick(this.canvas);
    let targetPosition = { x: 0, y: 0 };

    if (mouseClick) {
      // Convert screen coordinates to world coordinates
      const worldPos = this.camera.screenToWorld(mouseClick.x, mouseClick.y);
      targetPosition = worldPos;
    }

    // Apply client-side prediction (immediate movement)
    if (movement.x !== 0 || movement.y !== 0) {
      this.predictMovement(playerEntityId, movement, deltaTime);
    }

    // Send input to server at rate limit (20 Hz)
    const currentTime = performance.now();
    if (currentTime - this.lastInputSendTime >= this.inputSendInterval) {
      this.sendInput(movement, action, targetPosition);
      this.lastInputSendTime = currentTime;
    }
  }

  /**
   * Apply client-side prediction for movement
   */
  private predictMovement(
    playerEntityId: number,
    movement: { x: number; y: number },
    deltaTime: number
  ): void {
    // Update player position based on movement input
    const currentX = Position.x[playerEntityId];
    const currentY = Position.y[playerEntityId];

    if (currentX === undefined || currentY === undefined) {
      return;
    }

    // Calculate new position
    const speed = this.movementSpeed * deltaTime;
    const newX = currentX + movement.x * speed;
    const newY = currentY + movement.y * speed;

    // Update position (client prediction)
    Position.x[playerEntityId] = newX;
    Position.y[playerEntityId] = newY;

    // Update velocity for rendering
    Velocity.dx[playerEntityId] = movement.x * this.movementSpeed;
    Velocity.dy[playerEntityId] = movement.y * this.movementSpeed;
  }

  /**
   * Send PlayerInput message to server
   */
  private sendInput(
    movement: { x: number; y: number },
    action: number,
    targetPosition: { x: number; y: number }
  ): void {
    // Increment sequence number
    this.inputSequence++;

    // Get current timestamp
    const timestamp = BigInt(Date.now());

    // Create and send PlayerInput message
    const message = createPlayerInputMessage(
      this.inputSequence,
      timestamp,
      movement.x,
      movement.y,
      action,
      targetPosition.x,
      targetPosition.y
    );

    this.wsClient.send(message);

    // Debug logging
    if (config.debug && (movement.x !== 0 || movement.y !== 0 || action !== 0)) {
      console.log(
        `📤 Input #${this.inputSequence}: move=(${movement.x.toFixed(2)}, ${movement.y.toFixed(2)}) action=${action}`
      );
    }
  }

  /**
   * Enable input handling
   */
  public enable(): void {
    this.inputManager.enable();
  }

  /**
   * Disable input handling
   */
  public disable(): void {
    this.inputManager.disable();
  }
}
