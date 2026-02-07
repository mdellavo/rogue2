import { Camera } from './Camera';
import { config } from '@/config';
import type { TilesetManager } from './TilesetManager';

/**
 * WebGL 2D renderer with texture support for sprite-based rendering
 *
 * Features:
 * - Batched rendering for performance
 * - Texture-based sprite rendering
 * - Fallback to colored rectangles
 * - Automatic texture switching with batching
 */
export class RendererTextured {
  private canvas: HTMLCanvasElement;
  private gl: WebGLRenderingContext;
  private camera: Camera;
  private tilesetManager: TilesetManager;

  // Colored shader program (for rectangles and fallback)
  private colorProgram: WebGLProgram | null = null;
  private colorPositionBuffer: WebGLBuffer | null = null;
  private colorColorBuffer: WebGLBuffer | null = null;
  private colorPositionLocation: number = -1;
  private colorColorLocation: number = -1;
  private colorMatrixLocation: WebGLUniformLocation | null = null;

  // Textured shader program (for sprites)
  private textureProgram: WebGLProgram | null = null;
  private texturePositionBuffer: WebGLBuffer | null = null;
  private textureTexCoordBuffer: WebGLBuffer | null = null;
  private texturePositionLocation: number = -1;
  private textureTexCoordLocation: number = -1;
  private textureMatrixLocation: WebGLUniformLocation | null = null;
  private textureSamplerLocation: WebGLUniformLocation | null = null;

  // Batch rendering arrays for colored rendering
  private colorPositions: number[] = [];
  private colorColors: number[] = [];

  // Batch rendering arrays for textured rendering
  private texturePositions: number[] = [];
  private textureTexCoords: number[] = [];
  private currentTexture: WebGLTexture | null = null;

  private maxBatchSize: number = 10000;
  private readonly visionRadiusTiles: number = 20;

  constructor(canvas: HTMLCanvasElement, camera: Camera, tilesetManager: TilesetManager) {
    this.canvas = canvas;
    this.camera = camera;
    this.tilesetManager = tilesetManager;

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

    // Give tileset manager the GL context
    this.tilesetManager.setGLContext(gl);
  }

  /**
   * Initialize WebGL shaders and buffers
   */
  private initWebGL(): void {
    const gl = this.gl;

    // Initialize colored shader program
    this.initColorProgram();

    // Initialize textured shader program
    this.initTextureProgram();

    // Enable alpha blending
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
  }

  /**
   * Initialize colored rendering shader program
   */
  private initColorProgram(): void {
    const gl = this.gl;

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

    const fragmentShaderSource = `
      precision mediump float;

      varying vec4 v_color;

      void main() {
        gl_FragColor = v_color;
      }
    `;

    const vertexShader = this.compileShader(gl.VERTEX_SHADER, vertexShaderSource);
    const fragmentShader = this.compileShader(gl.FRAGMENT_SHADER, fragmentShaderSource);

    const program = gl.createProgram();
    if (!program) throw new Error('Failed to create color program');

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      const info = gl.getProgramInfoLog(program);
      throw new Error('Failed to link color program: ' + info);
    }

    this.colorProgram = program;
    this.colorPositionLocation = gl.getAttribLocation(program, 'a_position');
    this.colorColorLocation = gl.getAttribLocation(program, 'a_color');
    this.colorMatrixLocation = gl.getUniformLocation(program, 'u_matrix');
    this.colorPositionBuffer = gl.createBuffer();
    this.colorColorBuffer = gl.createBuffer();
  }

  /**
   * Initialize textured rendering shader program
   */
  private initTextureProgram(): void {
    const gl = this.gl;

    const vertexShaderSource = `
      attribute vec2 a_position;
      attribute vec2 a_texCoord;

      uniform mat3 u_matrix;

      varying vec2 v_texCoord;

      void main() {
        vec2 position = (u_matrix * vec3(a_position, 1.0)).xy;
        gl_Position = vec4(position, 0.0, 1.0);
        v_texCoord = a_texCoord;
      }
    `;

    const fragmentShaderSource = `
      precision mediump float;

      uniform sampler2D u_texture;
      varying vec2 v_texCoord;

      void main() {
        gl_FragColor = texture2D(u_texture, v_texCoord);
      }
    `;

    const vertexShader = this.compileShader(gl.VERTEX_SHADER, vertexShaderSource);
    const fragmentShader = this.compileShader(gl.FRAGMENT_SHADER, fragmentShaderSource);

    const program = gl.createProgram();
    if (!program) throw new Error('Failed to create texture program');

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      const info = gl.getProgramInfoLog(program);
      throw new Error('Failed to link texture program: ' + info);
    }

    this.textureProgram = program;
    this.texturePositionLocation = gl.getAttribLocation(program, 'a_position');
    this.textureTexCoordLocation = gl.getAttribLocation(program, 'a_texCoord');
    this.textureMatrixLocation = gl.getUniformLocation(program, 'u_matrix');
    this.textureSamplerLocation = gl.getUniformLocation(program, 'u_texture');
    this.texturePositionBuffer = gl.createBuffer();
    this.textureTexCoordBuffer = gl.createBuffer();
  }

  /**
   * Compile a shader
   */
  private compileShader(type: number, source: string): WebGLShader {
    const gl = this.gl;
    const shader = gl.createShader(type);
    if (!shader) throw new Error('Failed to create shader');

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
    gl.clearColor(0.1, 0.1, 0.15, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    // Clear batch arrays
    this.colorPositions = [];
    this.colorColors = [];
    this.texturePositions = [];
    this.textureTexCoords = [];
    this.currentTexture = null;

    // Draw checkerboard background using colored program
    const viewport = this.camera.getViewport();
    this.drawCheckerboard(viewport);
  }

  /**
   * Draw checkerboard background
   */
  private drawCheckerboard(viewport: {
    left: number;
    right: number;
    top: number;
    bottom: number;
  }): void {
    const tileSize = config.tileSize;
    const startTileX = Math.floor(viewport.left / tileSize);
    const endTileX = Math.ceil(viewport.right / tileSize);
    const startTileY = Math.floor(viewport.top / tileSize);
    const endTileY = Math.ceil(viewport.bottom / tileSize);

    const lightColor = { r: 0.25, g: 0.25, b: 0.3, a: 1.0 };
    const darkColor = { r: 0.15, g: 0.15, b: 0.2, a: 1.0 };

    for (let tileY = startTileY; tileY < endTileY; tileY++) {
      for (let tileX = startTileX; tileX < endTileX; tileX++) {
        const isLight = (tileX + tileY) % 2 === 0;
        const color = isLight ? lightColor : darkColor;
        const worldX = tileX * tileSize;
        const worldY = tileY * tileSize;

        this.drawRect(worldX, worldY, tileSize, tileSize, color.r, color.g, color.b, color.a);
      }
    }

    this.flushColorBatch();
  }

  /**
   * Draw a colored rectangle
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
    if (!this.camera.isRectVisible(x, y, width, height, config.tileSize)) {
      return;
    }

    // Add two triangles
    this.colorPositions.push(x, y, x + width, y, x, y + height);
    this.colorPositions.push(x + width, y, x + width, y + height, x, y + height);

    for (let i = 0; i < 6; i++) {
      this.colorColors.push(r, g, b, a);
    }

    if (this.colorPositions.length >= this.maxBatchSize * 12) {
      this.flushColorBatch();
    }
  }

  /**
   * Draw a sprite from a tileset
   */
  public drawSprite(
    tilesetName: string,
    spriteId: string,
    x: number,
    y: number,
    width: number,
    height: number
  ): void {
    if (!this.camera.isRectVisible(x, y, width, height, config.tileSize)) {
      return;
    }

    const sprite = this.tilesetManager.getSprite(tilesetName, spriteId);
    const texture = this.tilesetManager.getTexture(tilesetName);

    if (!sprite || !texture) {
      // Fallback to colored rect
      this.drawRect(x, y, width, height, 0.5, 0.5, 0.5, 1.0);
      return;
    }

    // Check if we need to switch textures (flush current batch)
    if (this.currentTexture && this.currentTexture !== texture) {
      this.flushTextureBatch();
    }
    this.currentTexture = texture;

    // Add two triangles with texture coordinates
    const { u0, v0, u1, v1 } = sprite;

    // Triangle 1: top-left, top-right, bottom-left
    this.texturePositions.push(x, y, x + width, y, x, y + height);
    this.textureTexCoords.push(u0, v0, u1, v0, u0, v1);

    // Triangle 2: top-right, bottom-right, bottom-left
    this.texturePositions.push(x + width, y, x + width, y + height, x, y + height);
    this.textureTexCoords.push(u1, v0, u1, v1, u0, v1);

    if (this.texturePositions.length >= this.maxBatchSize * 12) {
      this.flushTextureBatch();
    }
  }

  /**
   * Draw fog of war
   */
  public drawFogOfWar(playerX: number, playerY: number): void {
    const viewport = this.camera.getViewport();
    const fogColor = { r: 0.0, g: 0.0, b: 0.0, a: 0.7 };
    const visionRadiusPixels = this.visionRadiusTiles * config.tileSize;

    // Top rectangle
    if (viewport.top < playerY - visionRadiusPixels) {
      this.drawRect(
        viewport.left,
        viewport.top,
        viewport.right - viewport.left,
        playerY - visionRadiusPixels - viewport.top,
        fogColor.r,
        fogColor.g,
        fogColor.b,
        fogColor.a
      );
    }

    // Bottom rectangle
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

    // Left rectangle
    if (viewport.left < playerX - visionRadiusPixels) {
      this.drawRect(
        viewport.left,
        Math.max(viewport.top, playerY - visionRadiusPixels),
        playerX - visionRadiusPixels - viewport.left,
        Math.min(viewport.bottom, playerY + visionRadiusPixels) -
          Math.max(viewport.top, playerY - visionRadiusPixels),
        fogColor.r,
        fogColor.g,
        fogColor.b,
        fogColor.a
      );
    }

    // Right rectangle
    if (viewport.right > playerX + visionRadiusPixels) {
      this.drawRect(
        playerX + visionRadiusPixels,
        Math.max(viewport.top, playerY - visionRadiusPixels),
        viewport.right - (playerX + visionRadiusPixels),
        Math.min(viewport.bottom, playerY + visionRadiusPixels) -
          Math.max(viewport.top, playerY - visionRadiusPixels),
        fogColor.r,
        fogColor.g,
        fogColor.b,
        fogColor.a
      );
    }

    this.flushColorBatch();
  }

  /**
   * End frame and flush remaining batches
   */
  public endFrame(): void {
    this.flushTextureBatch();
    this.flushColorBatch();
  }

  /**
   * Flush colored batch to GPU
   */
  private flushColorBatch(): void {
    if (this.colorPositions.length === 0) return;

    const gl = this.gl;
    gl.useProgram(this.colorProgram);

    // Set projection matrix
    const viewport = this.camera.getViewport();
    const matrix = this.createProjectionMatrix(
      viewport.left,
      viewport.right,
      viewport.bottom,
      viewport.top
    );
    gl.uniformMatrix3fv(this.colorMatrixLocation, false, matrix);

    // Upload position data
    gl.bindBuffer(gl.ARRAY_BUFFER, this.colorPositionBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(this.colorPositions), gl.DYNAMIC_DRAW);
    gl.enableVertexAttribArray(this.colorPositionLocation);
    gl.vertexAttribPointer(this.colorPositionLocation, 2, gl.FLOAT, false, 0, 0);

    // Upload color data
    gl.bindBuffer(gl.ARRAY_BUFFER, this.colorColorBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(this.colorColors), gl.DYNAMIC_DRAW);
    gl.enableVertexAttribArray(this.colorColorLocation);
    gl.vertexAttribPointer(this.colorColorLocation, 4, gl.FLOAT, false, 0, 0);

    // Draw
    const vertexCount = this.colorPositions.length / 2;
    gl.drawArrays(gl.TRIANGLES, 0, vertexCount);

    // Clear batch
    this.colorPositions = [];
    this.colorColors = [];
  }

  /**
   * Flush textured batch to GPU
   */
  private flushTextureBatch(): void {
    if (this.texturePositions.length === 0 || !this.currentTexture) return;

    const gl = this.gl;
    gl.useProgram(this.textureProgram);

    // Set projection matrix
    const viewport = this.camera.getViewport();
    const matrix = this.createProjectionMatrix(
      viewport.left,
      viewport.right,
      viewport.bottom,
      viewport.top
    );
    gl.uniformMatrix3fv(this.textureMatrixLocation, false, matrix);

    // Bind texture
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, this.currentTexture);
    gl.uniform1i(this.textureSamplerLocation, 0);

    // Upload position data
    gl.bindBuffer(gl.ARRAY_BUFFER, this.texturePositionBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(this.texturePositions), gl.DYNAMIC_DRAW);
    gl.enableVertexAttribArray(this.texturePositionLocation);
    gl.vertexAttribPointer(this.texturePositionLocation, 2, gl.FLOAT, false, 0, 0);

    // Upload texture coordinate data
    gl.bindBuffer(gl.ARRAY_BUFFER, this.textureTexCoordBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(this.textureTexCoords), gl.DYNAMIC_DRAW);
    gl.enableVertexAttribArray(this.textureTexCoordLocation);
    gl.vertexAttribPointer(this.textureTexCoordLocation, 2, gl.FLOAT, false, 0, 0);

    // Draw
    const vertexCount = this.texturePositions.length / 2;
    gl.drawArrays(gl.TRIANGLES, 0, vertexCount);

    // Clear batch
    this.texturePositions = [];
    this.textureTexCoords = [];
    this.currentTexture = null;
  }

  /**
   * Create projection matrix
   */
  private createProjectionMatrix(
    left: number,
    right: number,
    bottom: number,
    top: number
  ): Float32Array {
    const width = right - left;
    const height = bottom - top;

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

  public resize(width: number, height: number): void {
    this.canvas.width = width;
    this.canvas.height = height;
    this.gl.viewport(0, 0, width, height);
  }

  public getCanvas(): HTMLCanvasElement {
    return this.canvas;
  }

  public getCamera(): Camera {
    return this.camera;
  }
}
