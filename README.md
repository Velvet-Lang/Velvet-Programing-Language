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

Velvet is a declarative, shell-like programming language with minimal syntax, C-like performance, and Rust-like memory safety. It uses symbols (`@`, `#`, `[ ]`, `{ }`) and supports embedding Python/shell code.

## Structure
- `/core`: Rust-based compiler (`velvetc`) using Pest for parsing.
- `/weave`: Rust-based CLI for project management, with cyberpunk UI (`crossterm`).
- `/vel`: Python-based runner for .o/.velvet files, using `rich`.
- Zig: Runtime and packaging (`build.zig`).

## Setup
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install Zig: Download from https://ziglang.org/download/
3. Install Python: `pip install rich` (for `vel/`)
4. Build: `cd core && cargo build; cd ../weave && cargo build; cd .. && zig build`
5. Run CLI: `cargo run --bin weave -- init`

## Commands
- `weave init`: Create `src/` and `weave.velvet`.
- `weave build --bin`: Compile and package to .velvet.
- `weave run`: Simulate execution (dev mode).
- `weave install <lib>`: Install dependencies.
- `vel <file.o|file.velvet>`: Execute compiled files.

## Example
See `src/main.vel` and `src/advanced.vel`.
