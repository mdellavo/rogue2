import { Camera } from './Camera';
import { config } from '@/config';

/**
 * Simple WebGL 2D renderer for sprite-based games
 * Currently renders colored rectangles as placeholders for sprites
 */
export class Renderer {
  private canvas: HTMLCanvasElement;
  private gl: WebGLRenderingContext;
  private camera: Camera;

  // Shader program and attributes
  private program: WebGLProgram | null = null;
  private positionBuffer: WebGLBuffer | null = null;
  private colorBuffer: WebGLBuffer | null = null;

  // Shader attribute/uniform locations
  private positionLocation: number = -1;
  private colorLocation: number = -1;
  private matrixLocation: WebGLUniformLocation | null = null;

  // Batch rendering arrays
  private positions: number[] = [];
  private colors: number[] = [];
  private maxBatchSize: number = 10000; // Maximum sprites per batch

  // Vision/fog of war
  private readonly visionRadiusTiles: number = 20; // Vision range in tiles

  constructor(canvas: HTMLCanvasElement, camera: Camera) {
    this.canvas = canvas;
    this.camera = camera;

    // Set canvas size
    this.canvas.width = config.viewportWidth;
    this.canvas.height = config.viewportHeight;

    // Get WebGL context
    const gl = canvas.getContext('webgl');
    if (!gl) {
      throw new Error('WebGL not supported');
    }
    this.gl = gl;

    // Initialize WebGL
    this.initWebGL();
  }

  /**
   * Initialize WebGL shaders and buffers
   */
  private initWebGL(): void {
    const gl = this.gl;

    // Vertex shader - transforms positions and passes color to fragment shader
    const vertexShaderSource = `
      attribute vec2 a_position;
      attribute vec4 a_color;

      uniform mat3 u_matrix;

      varying vec4 v_color;

      void main() {
        vec2 position = (u_matrix * vec3(a_position, 1.0)).xy;
        gl_Position = vec4(position, 0.0, 1.0);
        v_color = a_color;
      }
    `;

    // Fragment shader - applies color to each pixel
    const fragmentShaderSource = `
      precision mediump float;

      varying vec4 v_color;

      void main() {
        gl_FragColor = v_color;
      }
    `;

    // Compile shaders
    const vertexShader = this.compileShader(gl.VERTEX_SHADER, vertexShaderSource);
    const fragmentShader = this.compileShader(gl.FRAGMENT_SHADER, fragmentShaderSource);

    // Create and link program
    const program = gl.createProgram();
    if (!program) {
      throw new Error('Failed to create WebGL program');
    }

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      const info = gl.getProgramInfoLog(program);
      throw new Error('Failed to link WebGL program: ' + info);
    }

    this.program = program;

    // Get attribute and uniform locations
    this.positionLocation = gl.getAttribLocation(program, 'a_position');
    this.colorLocation = gl.getAttribLocation(program, 'a_color');
    this.matrixLocation = gl.getUniformLocation(program, 'u_matrix');

    // Create buffers
    this.positionBuffer = gl.createBuffer();
    this.colorBuffer = gl.createBuffer();

    // Enable alpha blending
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
  }

  /**
   * Compile a shader
   */
  private compileShader(type: number, source: string): WebGLShader {
    const gl = this.gl;
    const shader = gl.createShader(type);
    if (!shader) {
      throw new Error('Failed to create shader');
    }

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      const info = gl.getShaderInfoLog(shader);
      gl.deleteShader(shader);
      throw new Error('Failed to compile shader: ' + info);
    }

    return shader;
  }

  /**
   * Begin a new render frame
   */
  public beginFrame(): void {
    const gl = this.gl;

    // Clear the canvas
    gl.clearColor(0.1, 0.1, 0.15, 1.0); // Dark blue-gray background
    gl.clear(gl.COLOR_BUFFER_BIT);

    // Use the shader program
    gl.useProgram(this.program);

    // Set up the projection matrix (converts world coords to clip space)
    const viewport = this.camera.getViewport();
    const matrix = this.createProjectionMatrix(
      viewport.left,
      viewport.right,
      viewport.bottom,
      viewport.top
    );

    gl.uniformMatrix3fv(this.matrixLocation, false, matrix);

    // Clear batch arrays
    this.positions = [];
    this.colors = [];

    // Draw checkerboard background
    this.drawCheckerboard(viewport);
  }

  /**
   * Draw a checkerboard pattern as the background
   */
  private drawCheckerboard(viewport: {
    left: number;
    right: number;
    top: number;
    bottom: number;
  }): void {
    const tileSize = config.tileSize;

    // Calculate the tile boundaries visible in the viewport
    const startTileX = Math.floor(viewport.left / tileSize);
    const endTileX = Math.ceil(viewport.right / tileSize);
    const startTileY = Math.floor(viewport.top / tileSize);
    const endTileY = Math.ceil(viewport.bottom / tileSize);

    // Checkerboard colors
    const lightColor = { r: 0.25, g: 0.25, b: 0.3, a: 1.0 }; // Light gray-blue
    const darkColor = { r: 0.15, g: 0.15, b: 0.2, a: 1.0 }; // Dark gray-blue

    // Draw each tile
    for (let tileY = startTileY; tileY < endTileY; tileY++) {
      for (let tileX = startTileX; tileX < endTileX; tileX++) {
        // Determine checkerboard color based on tile coordinates
        const isLight = (tileX + tileY) % 2 === 0;
        const color = isLight ? lightColor : darkColor;

        // Calculate world position
        const worldX = tileX * tileSize;
        const worldY = tileY * tileSize;

        // Draw tile
        this.drawRect(
          worldX,
          worldY,
          tileSize,
          tileSize,
          color.r,
          color.g,
          color.b,
          color.a
        );
      }
    }

    // Flush checkerboard to GPU before drawing entities
    this.flush();
  }

  /**
   * Create a projection matrix for 2D rendering
   */
  private createProjectionMatrix(
    left: number,
    right: number,
    bottom: number,
    top: number
  ): Float32Array {
    const width = right - left;
    const height = bottom - top;

    // Orthographic projection matrix
    return new Float32Array([
      2 / width,
      0,
      0,
      0,
      -2 / height,
      0,
      -1 - (2 * left) / width,
      1 + (2 * top) / height,
      1,
    ]);
  }

  /**
   * Draw a colored rectangle (placeholder for sprite rendering)
   */
  public drawRect(
    x: number,
    y: number,
    width: number,
    height: number,
    r: number,
    g: number,
    b: number,
    a: number = 1.0
  ): void {
    // Check if visible (with padding for smooth scrolling)
    if (!this.camera.isRectVisible(x, y, width, height, config.tileSize)) {
      return;
    }

    // Add two triangles to form a rectangle
    // Triangle 1: top-left, top-right, bottom-left
    // Triangle 2: top-right, bottom-right, bottom-left

    // Triangle 1
    this.positions.push(x, y, x + width, y, x, y + height);

    // Triangle 2
    this.positions.push(x + width, y, x + width, y + height, x, y + height);

    // Colors for all 6 vertices (2 triangles * 3 vertices)
    for (let i = 0; i < 6; i++) {
      this.colors.push(r, g, b, a);
    }

    // Flush if batch is full
    if (this.positions.length >= this.maxBatchSize * 12) {
      this.flush();
    }
  }

  /**
   * Draw fog of war overlay (darkens areas outside vision radius)
   * Should be called after all entities are rendered
   */
  public drawFogOfWar(playerX: number, playerY: number): void {
    const viewport = this.camera.getViewport();
    const fogColor = { r: 0.0, g: 0.0, b: 0.0, a: 0.7 }; // Dark semi-transparent overlay

    // Calculate vision radius in pixels
    const visionRadiusPixels = this.visionRadiusTiles * config.tileSize;

    // Draw fog as rectangles covering areas outside the vision circle

    // Draw fog as a full-screen quad with transparency based on distance from player
    // We'll use multiple concentric rectangles to create a gradient effect

    // For now, draw a simple overlay outside the vision circle
    // We'll draw four rectangles covering areas outside the circle

    // Top rectangle (above vision circle)
    if (viewport.top < playerY - visionRadiusPixels) {
      this.drawRect(
        viewport.left,
        viewport.top,
        viewport.right - viewport.left,
        (playerY - visionRadiusPixels) - viewport.top,
        fogColor.r,
        fogColor.g,
        fogColor.b,
        fogColor.a
      );
    }

    // Bottom rectangle (below vision circle)
    if (viewport.bottom > playerY + visionRadiusPixels) {
      this.drawRect(
        viewport.left,
        playerY + visionRadiusPixels,
        viewport.right - viewport.left,
        viewport.bottom - (playerY + visionRadiusPixels),
        fogColor.r,
        fogColor.g,
        fogColor.b,
        fogColor.a
      );
    }

    // Left rectangle (left of vision circle)
    if (viewport.left < playerX - visionRadiusPixels) {
      this.drawRect(
        viewport.left,
        Math.max(viewport.top, playerY - visionRadiusPixels),
        (playerX - visionRadiusPixels) - viewport.left,
        Math.min(viewport.bottom, playerY + visionRadiusPixels) - Math.max(viewport.top, playerY - visionRadiusPixels),
        fogColor.r,
        fogColor.g,
        fogColor.b,
        fogColor.a
      );
    }

    // Right rectangle (right of vision circle)
    if (viewport.right > playerX + visionRadiusPixels) {
      this.drawRect(
        playerX + visionRadiusPixels,
        Math.max(viewport.top, playerY - visionRadiusPixels),
        viewport.right - (playerX + visionRadiusPixels),
        Math.min(viewport.bottom, playerY + visionRadiusPixels) - Math.max(viewport.top, playerY - visionRadiusPixels),
        fogColor.r,
        fogColor.g,
        fogColor.b,
        fogColor.a
      );
    }

    // Flush fog to GPU
    this.flush();
  }

  /**
   * End the render frame and flush remaining batches
   */
  public endFrame(): void {
    this.flush();
  }

  /**
   * Flush the current batch to the GPU
   */
  private flush(): void {
    if (this.positions.length === 0) {
      return;
    }

    const gl = this.gl;

    // Upload position data
    gl.bindBuffer(gl.ARRAY_BUFFER, this.positionBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(this.positions), gl.DYNAMIC_DRAW);
    gl.enableVertexAttribArray(this.positionLocation);
    gl.vertexAttribPointer(this.positionLocation, 2, gl.FLOAT, false, 0, 0);

    // Upload color data
    gl.bindBuffer(gl.ARRAY_BUFFER, this.colorBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(this.colors), gl.DYNAMIC_DRAW);
    gl.enableVertexAttribArray(this.colorLocation);
    gl.vertexAttribPointer(this.colorLocation, 4, gl.FLOAT, false, 0, 0);

    // Draw
    const vertexCount = this.positions.length / 2;
    gl.drawArrays(gl.TRIANGLES, 0, vertexCount);

    // Clear batch arrays
    this.positions = [];
    this.colors = [];
  }

  /**
   * Resize the canvas and update viewport
   */
  public resize(width: number, height: number): void {
    this.canvas.width = width;
    this.canvas.height = height;
    this.gl.viewport(0, 0, width, height);
  }

  /**
   * Get the canvas element
   */
  public getCanvas(): HTMLCanvasElement {
    return this.canvas;
  }

  /**
   * Get the camera
   */
  public getCamera(): Camera {
    return this.camera;
  }
}
