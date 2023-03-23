#!/bin/bash
set -e

# Install cargo if not already installed
if ! command -v cargo &>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Clone repository under /tmp and build binary
rm -rf /tmp/heygpt
cd /tmp
git clone https://github.com/p208p2002/heygpt.git
cd heygpt
cargo install --path .

echo "Successfully installed heygpt!"

# Cleanup
cd ..
rm -rf heygpt