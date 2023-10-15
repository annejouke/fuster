#!/bin/sh

curl -LO $(curl -s https://api.github.com/repos/annejouke/fuster/releases/latest | grep 'browser_' | grep macos.zip | sed 's/"browser_download_url": "\(.*\)"/\1/')

unzip macos.zip

rm macos.zip

mv fuster /usr/local/bin/fus

chmod +x /usr/local/bin/fus

echo "Fuster installed successfully!"
