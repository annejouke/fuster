#!/bin/sh

curl -LO https://github.com/annejouke/fuster/releases/download/v0.1.0/macos.zip

unzip macos.zip

rm macos.zip

mv fuster /usr/local/bin/fus

chmod +x /usr/local/bin/fus

echo "Fuster installed successfully!"
