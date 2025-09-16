import argparse
import sys
from rich.console import Console
from rich.text import Text
from .project import init_project, clear_project
from .deps import install_dep, install_in, install_o
from .build import build_project
from .run import run_dev

console = Console()

def cyber_print(text):
    styled = Text(text, style="bold green blink")  # Cyberpunk neon green
    console.print(styled)

def main():
    parser = argparse.ArgumentParser(description="Weave: Velvet Build Tool", formatter_class=argparse.RawTextHelpFormatter)
    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    subparsers.add_parser("init", help="Initialize a new Velvet project")
    subparsers.add_parser("clear", help="Clear build artifacts")
    subparsers.add_parser("help", help="Show this help message")
    install_parser = subparsers.add_parser("install", help="Install a dependency")
    install_parser.add_argument("lib", type=str, help="Library name")
    install_in_parser = subparsers.add_parser("install-in", help="Install dependency in project")
    install_in_parser.add_argument("lib", type=str, help="Library name")
    install_o_parser = subparsers.add_parser("install-o", help="Install dependency in isolated environment")
    install_o_parser.add_argument("lib", type=str, help="Library name")
    build_parser = subparsers.add_parser("build", help="Build the project")
    build_parser.add_argument("--deb", action="store_true", help="Build Debian package")
    build_parser.add_argument("--rpm", action="store_true", help="Build RPM package")
    build_parser.add_argument("--bin", action="store_true", help="Build binary")
    build_parser.add_argument("--appimage", action="store_true", help="Build AppImage")
    subparsers.add_parser("run", help="Run project in dev mode (text-based)")

    args = parser.parse_args()

    if args.command == "init":
        cyber_print("Initializing Velvet project...")
        init_project()
    elif args.command == "clear":
        cyber_print("Clearing project...")
        clear_project()
    elif args.command == "install":
        cyber_print(f"Installing {args.lib}...")
        install_dep(args.lib)
    elif args.command == "install-in":
        cyber_print(f"Installing {args.lib} in project...")
        install_in(args.lib)
    elif args.command == "install-o":
        cyber_print(f"Installing {args.lib} in isolated env...")
        install_o(args.lib)
    elif args.command == "build":
        cyber_print("Building project...")
        options = []
        if args.deb: options.append("deb")
        if args.rpm: options.append("rpm")
        if args.bin: options.append("bin")
        if args.appimage: options.append("appimage")
        build_project(options)
    elif args.command == "run":
        cyber_print("Running in dev mode...")
        run_dev()
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
