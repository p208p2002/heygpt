#!/bin/bash
# Check CMD exist
if ! [ -x "$(command -v tar)" ]; then
  echo 'Error: tar is not installed.' >&2
  exit 1
fi

if ! [ -x "$(command -v curl)" ]; then
  echo 'Error: curl is not installed.' >&2
  exit 1
fi

# 
rm -rf /tmp/heygpt

#
cd /tmp
_ARCH=$(uname -m)
_ID=$(id -u)

#
if [[ "$OSTYPE" == "linux-gnu"* && "$_ARCH" == "x86_64" ]]; then 
    # corrected URL
    curl -o heygpt.tar.gz -LJ https://github.com/p208p2002/heygpt/releases/download/latest-Linux-X64/heygpt-Linux-X64.tar.gz
    
elif [[ "$OSTYPE" == "darwin"* && "$_ARCH" == "x86_64"  ]]; then
    # corrected URL
    curl -o heygpt.tar.gz -LJ https://github.com/p208p2002/heygpt/releases/download/latest-macOS-X64/heygpt-macOS-X64.tar.gz
    
elif [[ "$OSTYPE" == "darwin"* && "$_ARCH" == "arm64"  ]]; then
    # corrected URL
    curl -o heygpt.tar.gz -LJ https://github.com/p208p2002/heygpt/releases/download/latest-macOS-ARM64/heygpt-macOS-ARM64.tar.gz
else
    echo "Heygpt not available for this computer"
    exit 1
fi

tar -xzf heygpt.tar.gz

if [[ "$_ID" == 0 ]]; then 
    cp target/release/heygpt /usr/local/bin 
else
    sudo cp target/release/heygpt /usr/local/bin
fi

rm -rf /tmp/heygpt.tar.gz
rm -rf /tmp/heygpt
