#!/bin/bash
set -e

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