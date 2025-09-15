#!/bin/bash
echo "[INFO] Downloading binary file..."
curl -L -o /tmp/weave https://github.com/Velvet-Lang/Velvet-Programing-Language/releases/download/v0.1/weave
echo "[INFO] Moving the binary to /usr/bin"
sudo mv /tmp/weave /usr/bin/
sudo chmod a+x /usr/bin/weave
echo "[DONE] Now run weave help"
for i in $(seq 10 -1 1); do
    echo -ne "$i seconds to clear\r"
    sleep 1
done
clear
