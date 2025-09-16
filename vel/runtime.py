import os
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
from rich.console import Console

console = Console()

def load_o(file):
    console.print(f"[cyan]Loading {file}...[/]")
    # Dynamic load .o (ctypes)
    import ctypes
    return ctypes.CDLL(file)

def load_velvet(file):
    console.print(f"[cyan]Loading {file}...[/]")
    # Unpack and load (stub)
    pass

class ReloadHandler(FileSystemEventHandler):
    def __init__(self, file, interpreter):
        self.file = file
        self.interpreter = interpreter

    def on_modified(self, event):
        if event.src_path.endswith(self.file):
            console.print("[yellow]File changed, reloading...[/]")
            self.interpreter(self.file)

def hot_reload(file, interpreter):
    event_handler = ReloadHandler(file, interpreter)
    observer = Observer()
    observer.schedule(event_handler, path=os.path.dirname(file), recursive=False)
    observer.start()
    console.print("[cyan]Hot reload enabled. Press Ctrl+C to stop.[/]")
    try:
        while True:
            pass
    except KeyboardInterrupt:
        observer.stop()
    observer.join()
