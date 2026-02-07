#!/usr/bin/env python3

import sys
import json
import collections

id_counts = collections.defaultdict(lambda: 1)


def convert(tileset):
    for tile in tileset["tiles"]:

        tile_id = tile['id'].replace("_", "-")
        id_count = id_counts[tile_id]
        id_counts[tile_id] += 1
        tile["id"] = f"{tile_id}-{id_count}"


obj = json.load(sys.stdin)
convert(obj)
json.dump(obj, sys.stdout)