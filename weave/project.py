import os
from rich.console import Console

console = Console()

def init_project():
    os.makedirs("src", exist_ok=True)
    with open("weave.velvet", "w") as f:
        f.write("<Project Name>\n[Dependences]\n[BUILD]\n")
    console.print("[green]Project initialized with weave.velvet and src/[/]")

def clear_project():
    for file in os.listdir("."):
        if file.endswith((".o", ".velvet")):
            os.remove(file)
    console.print("[green]Project cleared[/]")
