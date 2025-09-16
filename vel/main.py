import sys
from rich.console import Console
from .interpreter import interpret_o, interpret_velvet
from .runtime import load_o, load_velvet, hot_reload

console = Console(style="bold magenta")  # Cyberpunk purple

def main():
    if len(sys.argv) < 2:
        console.print("[red]Usage: vel <file.o|file.velvet> [--hot-reload][/]")
        sys.exit(1)

    file = sys.argv[1]
    hot = "--hot-reload" in sys.argv

    if file.endswith(".o"):
        console.print(f"[cyan]Executing {file}...[/]")
        if hot:
            hot_reload(file, interpret_o)
        else:
            interpret_o(file)
    elif file.endswith(".velvet"):
        console.print(f"[cyan]Executing {file}...[/]")
        if hot:
            hot_reload(file, interpret_velvet)
        else:
            interpret_velvet(file)
    else:
        console.print("[red]Invalid file type[/]")
        sys.exit(1)

if __name__ == "__main__":
    main()
