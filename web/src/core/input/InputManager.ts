/**
 * Tracks keyboard and mouse input state
 */
export class InputManager {
  // Keyboard state
  private keys: Set<string> = new Set();

  // Mouse state
  private mouseX: number = 0;
  private mouseY: number = 0;
  private mouseButtons: Set<number> = new Set();
  private lastMouseClick: { x: number; y: number; button: number } | null = null;

  // Action state
  private actionPressed: boolean = false;
  private actionType: number = 0; // 0=none, 1=attack, 2=interact

  // Input enabled flag
  private enabled: boolean = false;

  constructor() {
    this.setupEventListeners();
  }

  /**
   * Set up DOM event listeners
   */
  private setupEventListeners(): void {
    // Keyboard events
    window.addEventListener('keydown', this.handleKeyDown.bind(this));
    window.addEventListener('keyup', this.handleKeyUp.bind(this));

    // Mouse events
    window.addEventListener('mousemove', this.handleMouseMove.bind(this));
    window.addEventListener('mousedown', this.handleMouseDown.bind(this));
    window.addEventListener('mouseup', this.handleMouseUp.bind(this));

    // Prevent context menu on right-click
    window.addEventListener('contextmenu', (e) => e.preventDefault());

    // Blur event to clear input when window loses focus
    window.addEventListener('blur', this.clearInput.bind(this));
  }

  /**
   * Handle keydown event
   */
  private handleKeyDown(event: KeyboardEvent): void {
    if (!this.enabled) return;

    const key = event.key.toLowerCase();
    this.keys.add(key);

    // Handle action keys
    if (key === ' ') {
      event.preventDefault();
      this.actionPressed = true;
      this.actionType = 1; // Attack
    } else if (key === 'e') {
      event.preventDefault();
      this.actionPressed = true;
      this.actionType = 2; // Interact
    }
  }

  /**
   * Handle keyup event
   */
  private handleKeyUp(event: KeyboardEvent): void {
    if (!this.enabled) return;

    const key = event.key.toLowerCase();
    this.keys.delete(key);

    // Clear action state
    if (key === ' ' || key === 'e') {
      this.actionPressed = false;
      this.actionType = 0;
    }
  }

  /**
   * Handle mousemove event
   */
  private handleMouseMove(event: MouseEvent): void {
    if (!this.enabled) return;

    this.mouseX = event.clientX;
    this.mouseY = event.clientY;
  }

  /**
   * Handle mousedown event
   */
  private handleMouseDown(event: MouseEvent): void {
    if (!this.enabled) return;

    this.mouseButtons.add(event.button);

    // Left click = attack/move
    if (event.button === 0) {
      this.lastMouseClick = {
        x: event.clientX,
        y: event.clientY,
        button: event.button,
      };
      this.actionPressed = true;
      this.actionType = 1; // Attack
    }
  }

  /**
   * Handle mouseup event
   */
  private handleMouseUp(event: MouseEvent): void {
    if (!this.enabled) return;

    this.mouseButtons.delete(event.button);

    // Clear action on left click release
    if (event.button === 0) {
      this.actionPressed = false;
      this.actionType = 0;
    }
  }

  /**
   * Get movement vector from WASD input
   * Returns normalized direction vector
   */
  public getMovementVector(): { x: number; y: number } {
    let x = 0;
    let y = 0;

    // WASD movement
    if (this.keys.has('w')) y -= 1; // Up
    if (this.keys.has('s')) y += 1; // Down
    if (this.keys.has('a')) x -= 1; // Left
    if (this.keys.has('d')) x += 1; // Right

    // Normalize diagonal movement
    if (x !== 0 && y !== 0) {
      const length = Math.sqrt(x * x + y * y);
      x /= length;
      y /= length;
    }

    return { x, y };
  }

  /**
   * Get current action state
   */
  public getAction(): number {
    return this.actionType;
  }

  /**
   * Check if action was just pressed (and consume it)
   */
  public consumeAction(): number {
    const action = this.actionType;
    if (this.actionPressed) {
      this.actionPressed = false;
      this.actionType = 0;
      return action;
    }
    return 0;
  }

  /**
   * Get mouse position relative to canvas
   */
  public getMousePosition(canvas: HTMLCanvasElement): { x: number; y: number } {
    const rect = canvas.getBoundingClientRect();
    return {
      x: this.mouseX - rect.left,
      y: this.mouseY - rect.top,
    };
  }

  /**
   * Get and consume last mouse click
   */
  public consumeMouseClick(canvas: HTMLCanvasElement): { x: number; y: number } | null {
    if (!this.lastMouseClick) {
      return null;
    }

    const rect = canvas.getBoundingClientRect();
    const click = {
      x: this.lastMouseClick.x - rect.left,
      y: this.lastMouseClick.y - rect.top,
    };

    this.lastMouseClick = null;
    return click;
  }

  /**
   * Check if a specific key is pressed
   */
  public isKeyPressed(key: string): boolean {
    return this.keys.has(key.toLowerCase());
  }

  /**
   * Clear all input state
   */
  public clearInput(): void {
    this.keys.clear();
    this.mouseButtons.clear();
    this.actionPressed = false;
    this.actionType = 0;
    this.lastMouseClick = null;
  }

  /**
   * Enable input handling
   */
  public enable(): void {
    this.enabled = true;
  }

  /**
   * Disable input handling
   */
  public disable(): void {
    this.enabled = false;
    this.clearInput();
  }

  /**
   * Check if input is enabled
   */
  public isEnabled(): boolean {
    return this.enabled;
  }
}
