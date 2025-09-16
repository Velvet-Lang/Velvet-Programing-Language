import os
import subprocess
from rich.console import Console

console = Console()

def build_project(options):
    # Parse weave.velvet
    with open("weave.velvet", "r") as f:
        config = f.read()
    # Collect .vel files
    vel_files = [f for f in os.listdir("src") if f.endswith(".vel")]
    if not vel_files:
        console.print("[red]No .vel files found in src/[/]")
        return

    # Compile each .vel to .o using Rust
    os.makedirs("build", exist_ok=True)
    for vel in vel_files:
        output = f"build/{vel}.o"
        console.print(f"[cyan]Compiling {vel} to {output}...[/]")
        result = subprocess.run(["cargo", "run", "--bin", "velvetc", f"src/{vel}", "-o", output], cwd="../core")
        if result.returncode != 0:
            console.print("[red]Compilation failed[/]")
            return

    # Link with Zig
    console.print("[cyan]Packaging with Zig...[/]")
    result = subprocess.run(["zig", "build"], cwd="..")
    if result.returncode != 0:
        console.print("[red]Packaging failed[/]")
        return

    # Handle extra build options (deb, rpm, etc.)
    for opt in options:
        console.print(f"[cyan]Building {opt} package...[/]")
        # Placeholder: Use tools like fpm for deb/rpm
    console.print("[green]Build complete[/]")
