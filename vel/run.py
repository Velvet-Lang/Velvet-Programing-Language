# vel/run.py
import subprocess
import re
from core_ffi import parse_and_run  # Zakładamy Python bindings do Rust via pyo3 (dodaj w Cargo.toml: pyo3 = "*")

def run_velvet(file_path):
    with open(file_path, 'r') as f:
        code = f.read()
    # Uproszczone: parse via Rust FFI (w realu: użyj pyo3 lub ctypes)
    # Dla shell-like: użyj subprocess z sandbox (np. firejail dla bezpieczeństwa)
    try:
        # Parse i run via Rust
        result = parse_and_run(code)  # FFI call
        print(result)
    except Exception as e:
        print(f"Runtime error: {e}")
        # Fallback: shell exec z ograniczeniami
        cmd = re.sub(r'[@<>].*', '', code)  # Strip comments/redirs
        subprocess.run(cmd, shell=True, check=True, capture_output=True)
