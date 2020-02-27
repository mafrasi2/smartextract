#!/usr/bin/env python3

import json
from pathlib import Path

def describe(directory):
    desc = {}
    for entry in directory.iterdir():
        if entry.is_dir():
            desc[entry.name] = describe(entry)
        else:
            desc[entry.name] = entry.stat().st_size
    return desc

if __name__ == "__main__":
    path = Path()
    desc = describe(path)

    with open("description.json", "w") as f:
        json.dump(desc, f, indent="  ")
