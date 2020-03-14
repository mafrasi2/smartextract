#!/usr/bin/env python3

import hashlib
import json
from pathlib import Path

def describe(directory):
    desc = {}
    for entry in directory.iterdir():
        if entry.is_dir():
            desc[entry.name] = describe(entry)
        else:
            hsh = hashlib.blake2b()
            with open(entry, "rb") as f:
                while True:
                    chunk = f.read(4096)
                    if not chunk:
                        break
                    hsh.update(chunk)
            desc[entry.name] = hsh.hexdigest()
    return desc

if __name__ == "__main__":
    path = Path()
    desc = describe(path)

    with open("description.json", "w") as f:
        json.dump(desc, f, indent="  ")
