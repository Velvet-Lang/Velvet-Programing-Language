from rich.console import Console

console = Console()

def run_dev():
    console.print("[cyan]Running in dev mode (text-based, no compilation)...[/]")
    # Placeholder: Parse .vel files and simulate execution
    for file in os.listdir("src"):
        if file.endswith(".vel"):
            with open(f"src/{file}", "r") as f:
                content = f.read()
                console.print(f"[magenta]Simulating {file}:[/] {content}")
