#!/bin/bash
echo "[CHECKING AND COMPILING...]"
echo "[RUN] Cloning git repo."
git clone https://github.com/Velvet-Lang/Velvet-Programing-Language.git /tmp/
cd /tmp/Velvet-Programing-Language/
echo "[INFO] INSTALL TOOLS"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2

stable
complete
y
1
echo "[BUILD] Building Binars..."
cd /tmp/Velvet-Programing-Language/
cargo build --release
zig build
echo "[MOVE] Transfering Binars..."
sudo mv /tmp/Velvet-Programing-Language/target/release/weave /usr/bin/
echo "[INFO] Operation Complete :D - now run weave help or vel help"
