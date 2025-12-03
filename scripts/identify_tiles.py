#!/usr/bin/env python3
"""
Tileset Sprite Identifier

This script analyzes a tileset image and uses Claude's vision API to identify
each sprite, generating a JSON manifest with names, descriptions, and metadata.

Usage:
    python identify_tiles.py tileset_config.yaml

The YAML config should contain:
    image: path/to/tileset.png
    tileSize: 64
    gridWidth: 16   # number of tiles horizontally
    gridHeight: 16  # number of tiles vertically

Output:
    sprites.json - JSON manifest with sprite metadata
"""

import sys
import yaml
import json
import base64
from pathlib import Path
from PIL import Image
from anthropic import Anthropic

def load_config(config_path: str) -> dict:
    """Load tileset configuration from YAML file."""
    with open(config_path, 'r') as f:
        return yaml.safe_load(f)

def extract_tile(image: Image.Image, x: int, y: int, tile_size: int) -> Image.Image:
    """Extract a single tile from the tileset."""
    left = x * tile_size
    top = y * tile_size
    right = left + tile_size
    bottom = top + tile_size
    return image.crop((left, top, right, bottom))

def is_empty_tile(tile: Image.Image, threshold: int = 10) -> bool:
    """Check if a tile is mostly empty/transparent."""
    if tile.mode != 'RGBA':
        tile = tile.convert('RGBA')

    # Count non-transparent pixels
    pixels = list(tile.getdata())
    non_transparent = sum(1 for _, _, _, a in pixels if a > threshold)

    # Consider empty if less than 5% of pixels are visible
    return non_transparent < (len(pixels) * 0.05)

def encode_image_base64(tile: Image.Image) -> str:
    """Encode a PIL Image as base64 PNG."""
    from io import BytesIO
    buffer = BytesIO()
    tile.save(buffer, format='PNG')
    return base64.b64encode(buffer.getvalue()).decode('utf-8')

def identify_sprite(client: Anthropic, tile: Image.Image, grid_x: int, grid_y: int) -> dict:
    """Use Claude's vision API to identify and describe a sprite."""

    # Encode tile as base64
    image_data = encode_image_base64(tile)

    # Create prompt for Claude
    prompt = f"""Analyze this 64x64 pixel sprite from a roguelike game tileset (position: row {grid_y}, col {grid_x}).

Please provide:
1. A short descriptive ID (lowercase, underscores, e.g., "stone_wall", "wooden_door", "grass_floor")
2. A brief name (e.g., "Stone Wall", "Wooden Door", "Grass Floor")
3. A one-sentence description
4. The type (choose one: "floor", "wall", "door", "decoration", "character", "item", "other")
5. Whether it's walkable (true/false)

Respond ONLY with a JSON object in this exact format:
{{
  "id": "descriptive_id",
  "name": "Display Name",
  "description": "Brief description of the sprite.",
  "type": "floor",
  "walkable": true
}}"""

    # Call Claude API
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1024,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": "image/png",
                            "data": image_data,
                        },
                    },
                    {
                        "type": "text",
                        "text": prompt
                    }
                ],
            }
        ],
    )

    # Parse response
    response_text = message.content[0].text.strip()

    # Extract JSON from response (Claude might wrap it in markdown code blocks)
    if response_text.startswith('```'):
        # Remove markdown code blocks
        lines = response_text.split('\n')
        response_text = '\n'.join(lines[1:-1])

    return json.loads(response_text)

def main():
    if len(sys.argv) != 2:
        print("Usage: python identify_tiles.py tileset_config.yaml")
        sys.exit(1)

    config_path = sys.argv[1]

    # Load configuration
    print(f"📂 Loading config from {config_path}...")
    config = load_config(config_path)

    image_path = config['image']
    tile_size = config['tileSize']
    grid_width = config['gridWidth']
    grid_height = config['gridHeight']

    print(f"📸 Loading tileset: {image_path}")
    print(f"   Tile size: {tile_size}x{tile_size}")
    print(f"   Grid: {grid_width}x{grid_height} ({grid_width * grid_height} tiles)")

    # Load tileset image
    tileset = Image.open(image_path)

    # Initialize Anthropic client
    client = Anthropic()

    # Process each tile
    sprites = []
    total_tiles = grid_width * grid_height
    processed = 0
    skipped = 0

    print(f"\n🔍 Analyzing tiles...")

    for y in range(grid_height):
        for x in range(grid_width):
            tile_num = y * grid_width + x
            print(f"   [{tile_num + 1}/{total_tiles}] Processing tile at ({x}, {y})...", end=' ')

            # Extract tile
            tile = extract_tile(tileset, x, y, tile_size)
            tile.save(f"tiles/tile_{y}_{x}.png")

            # Skip empty tiles
            if is_empty_tile(tile):
                print("⊘ Empty, skipping")
                skipped += 1
                continue

            # Identify sprite using Claude
            try:
                sprite_data = identify_sprite(client, tile, x, y)

                # Add grid position
                sprite_data['gridX'] = x
                sprite_data['gridY'] = y

                sprites.append(sprite_data)
                processed += 1

                print(f"✓ {sprite_data['id']}")

            except Exception as e:
                print(f"✗ Error: {e}")
                # Add placeholder for failed tiles
                sprites.append({
                    "id": f"tile_{y}_{x}",
                    "name": f"Tile {tile_num}",
                    "description": "Failed to identify",
                    "type": "other",
                    "walkable": False,
                    "gridX": x,
                    "gridY": y
                })

    # Save results
    output_path = "sprites.json"
    print(f"\n💾 Writing results to {output_path}...")

    with open(output_path, 'w') as f:
        json.dump({
            "tileSize": tile_size,
            "gridWidth": grid_width,
            "gridHeight": grid_height,
            "sprites": sprites
        }, f, indent=2)

    print(f"\n✅ Done!")
    print(f"   Processed: {processed} tiles")
    print(f"   Skipped: {skipped} empty tiles")
    print(f"   Total: {len(sprites)} sprites in manifest")

if __name__ == '__main__':
    main()
