#!/bin/bash
echo "[CHECKING AND COMPILING...]"
echo "[RUN] Cloning git repo."
git clone https://github.com/Velvet-Lang/Velvet-Programing-Language.git /tmp/
cd /tmp/Velvet-Programing-Language/
echo "[INFO] INSTALL TOOLS"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2

stable
echo "[BUILD] Building Binars..."
echo "[MOVE] Transfering Binaras.."
sudo mv /tmp/Velvet-Programing-Language/target/release/weave /usr/bin/
sudo 
