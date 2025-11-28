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
