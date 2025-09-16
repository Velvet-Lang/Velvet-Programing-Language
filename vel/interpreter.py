import ctypes
import asyncio
from rich.console import Console
from ruamel.yaml import YAML
import toml
import json

console = Console()

async def interpret_o(file):
    console.print(f"[cyan]Interpreting {file} (object file)...[/]")
    # Load .o with ctypes for FFI
    lib = ctypes.CDLL(file)
    lib._start()  # Call assembly entry
    # Async handling (stub for spawn)
    loop = asyncio.get_event_loop()
    await loop.run_in_executor(None, lambda: print("Async task"))
    # Serialization example
    data = {"key": "value"}
    with open("output.json", "w") as f:
        json.dump(data, f)
    with open("output.toml", "w") as f:
        toml.dump(data, f)
    yaml = YAML()
    with open("output.yaml", "w") as f:
        yaml.dump(data, f)
    console.print("[green]Execution complete[/]")

async def interpret_velvet(file):
    console.print(f"[cyan]Interpreting {file} (Velvet package)...[/]")
    # Call Zig runtime (assume velvet-runtime executable)
    import subprocess
    subprocess.run(["./velvet-runtime", file])
    console.print("[green]Execution complete[/]")
