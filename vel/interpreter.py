from rich.console import Console

console = Console()

def interpret_o(file):
    console.print(f"[cyan]Interpreting {file} (object file)...[/]")
    # Placeholder: Use ctypes or similar to load and execute .o
    console.print("[green]Execution complete (stub)[/]")

def interpret_velvet(file):
    console.print(f"[cyan]Interpreting {file} (Velvet package)...[/]")
    # Placeholder: Call Zig runtime to execute .velvet
    console.print("[green]Execution complete (stub)[/]")
