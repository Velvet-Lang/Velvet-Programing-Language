#!/usr/bin/env python3
# vel/vel.py
import sys
import subprocess
import os
from run import run_velvet
from init import init_project
from help import show_help

def main():
    if len(sys.argv) < 2:
        show_help()
        sys.exit(1)

    cmd = sys.argv[1]
    if cmd == "init":
        init_project()
    elif cmd == "run":
        if len(sys.argv) < 3:
            print("Usage: vel run <file.vel>")
            sys.exit(1)
        run_velvet(sys.argv[2])
    elif cmd == "help":
        show_help()
    else:
        print(f"Unknown command: {cmd}")

if __name__ == "__main__":
    main()
