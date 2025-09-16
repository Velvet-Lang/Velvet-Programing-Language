# Velvet-Programing-Language

downlaod velvet - for linux:
```bash
curl -L -o /tmp/install.sh https://raw.githubusercontent.com/Velvet-Lang/Velvet-Programing-Language/main/install/install.sh
````
download velvet - for Windows:
```bash
curl -L -o "$env:TEMP\install.sh" https://raw.githubusercontent.com/Velvet-Lang/Velvet-Programing-Language/main/install/install.sh
````

# about
Velvet is a declarative, shell-like programming language with minimal syntax, high performance (C-like), and memory safety (Rust-like). It uses symbols (`@`, `#`, `[ ]`, `{ }`) for simplicity and supports embedding code from other languages (e.g., Python, shell).

## Structure
- `/core`: Rust-based compiler (`velvetc`) for parsing, semantic analysis, optimization, and .o file generation.
- `/weave`: Python-based CLI for project management and builds, with a cyberpunk aesthetic.
- `/vel`: Python-based runner for .o and .velvet files, using `rich` for styled output.
- Zig: Runtime and packaging, linking .o files into .velvet executables.

## Setup
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install Zig: Download from https://ziglang.org/download/
3. Install Python: `pip install -r requirements.txt` (includes `rich`)
4. Build: `cd core && cargo build; cd .. && zig build`
5. Run CLI: `python -m weave init`

## Commands
- `weave init`: Create project structure (`src/`, `weave.velvet`).
- `weave build`: Compile .vel to .o (Rust) and package to .velvet (Zig).
- `weave run`: Run in dev mode (text-based, no compile).
- `weave install <lib>`: Install dependencies.
- `vel <file.o|file.velvet>`: Execute compiled files.

## Example
```velvet
[Zależność] @ Declare dep
#python {
print("Hello from Python")
}
[Output] > "Hello, Velvet!" | print
