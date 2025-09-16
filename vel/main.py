import sys
from rich.console import Console
from .interpreter import interpret_o, interpret_velvet

console = Console(style="bold magenta")  # Cyberpunk purple

def main():
    if len(sys.argv) < 2:
        console.print("[red]Usage: vel <file.o|file.velvet>[/]")
        sys.exit(1)

    file = sys.argv[1]
    if file.endswith(".o"):
        console.print(f"[cyan]Executing {file}...[/]")
        interpret_o(file)
    elif file.endswith(".velvet"):
        console.print(f"[cyan]Executing {file}...[/]")
        interpret_velvet(file)
    else:
        console.print("[red]Invalid file type: Must be .o or .velvet[/]")
        sys.exit(1)

if __name__ == "__main__":
    main()
