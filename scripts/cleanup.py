#!/usr/bin/env python3

import sys
import json
import collections

obj = json.load(sys.stdin)


id_counts = collections.defaultdict(lambda: 1)

def convert(obj):
    sprites = obj["sprites"]

    tiles = []
    for sprite in sprites:

        sprite_id = sprite['id'].replace("_", "-")
        id_count = id_counts[sprite_id]
        id_counts[sprite_id] += 1

        tile ={
          "id": f"{sprite_id}-{id_count}",
          "page": "dungeon",
          "name": sprite["name"],
          "row": sprite["gridY"],
          "col": sprite["gridX"],
          "x": sprite["gridX"] * obj["tileSize"],
          "y": sprite["gridY"] * obj["tileSize"],
          "w": obj["tileSize"],
          "h": obj["tileSize"],
          "type": sprite["type"],
          "walkable": sprite["walkable"],
          "description": sprite["description"],
        }
        tiles.append(tile)

    rv = {
        "meta": {
            "tileWidth": obj["tileSize"],
            "tileHeight": obj["tileSize"],
            "origin": "top-left",
            "pages": [
                {
                    "id": "dungeon",
                    "file": "tileset_dungeon.png",
                    "cols": obj["gridWidth"],
                    "rows": obj["gridHeight"]
                },

            ],
            "note": "x,y are computed as col*tileWidth, row*tileHeight on each page."
        },
        "tiles": tiles
    }
    return rv


rv = convert(obj)

json.dump(rv, sys.stdout)