import { config } from '@/config';

export class Camera {
  // Camera position in world coordinates (pixels)
  public x: number = 0;
  public y: number = 0;

  // Viewport dimensions (pixels)
  public readonly width: number;
  public readonly height: number;

  // Target position for smooth camera movement
  private targetX: number = 0;
  private targetY: number = 0;

  // Camera smoothing factor (0 = instant, 1 = no movement)
  private smoothing: number = 0.1;

  constructor(width: number = config.viewportWidth, height: number = config.viewportHeight) {
    this.width = width;
    this.height = height;
  }

  /**
   * Set the camera target (usually the player position)
   */
  public setTarget(x: number, y: number): void {
    this.targetX = x;
    this.targetY = y;
  }

  /**
   * Update camera position with smooth interpolation
   */
  public update(deltaTime: number): void {
    // Smooth camera movement using exponential interpolation
    const factor = 1 - Math.pow(this.smoothing, deltaTime * 60);

    this.x += (this.targetX - this.x) * factor;
    this.y += (this.targetY - this.y) * factor;
  }

  /**
   * Get the viewport bounds in world coordinates
   */
  public getViewport(): { left: number; top: number; right: number; bottom: number } {
    const halfWidth = this.width / 2;
    const halfHeight = this.height / 2;

    return {
      left: this.x - halfWidth,
      top: this.y - halfHeight,
      right: this.x + halfWidth,
      bottom: this.y + halfHeight,
    };
  }

  /**
   * Convert world coordinates to screen coordinates
   */
  public worldToScreen(worldX: number, worldY: number): { x: number; y: number } {
    const viewport = this.getViewport();
    return {
      x: worldX - viewport.left,
      y: worldY - viewport.top,
    };
  }

  /**
   * Convert screen coordinates to world coordinates
   */
  public screenToWorld(screenX: number, screenY: number): { x: number; y: number } {
    const viewport = this.getViewport();
    return {
      x: screenX + viewport.left,
      y: screenY + viewport.top,
    };
  }

  /**
   * Check if a position is visible in the viewport
   */
  public isVisible(x: number, y: number, padding: number = 0): boolean {
    const viewport = this.getViewport();
    return (
      x >= viewport.left - padding &&
      x <= viewport.right + padding &&
      y >= viewport.top - padding &&
      y <= viewport.bottom + padding
    );
  }

  /**
   * Check if a rectangle is visible in the viewport
   */
  public isRectVisible(
    x: number,
    y: number,
    width: number,
    height: number,
    padding: number = 0
  ): boolean {
    const viewport = this.getViewport();
    return (
      x + width >= viewport.left - padding &&
      x <= viewport.right + padding &&
      y + height >= viewport.top - padding &&
      y <= viewport.bottom + padding
    );
  }
}
