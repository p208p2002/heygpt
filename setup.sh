#!/bin/bash

cd /tmp
_ARCH=$(uname -m)

if [[ "$OSTYPE" == "linux-gnu"* && "$_ARCH" == "x86_64" ]]; then 
    wget https://github.com/p208p2002/heygpt/releases/download/latest-Linux-X64/heygpt-Linux-X64.tar.gz
    tar -xzf heygpt-Linux-X64.tar.gz
elif [[ "$OSTYPE" == "darwin"* && "$_ARCH" == "x86_64"  ]]; then
    wget https://github.com/p208p2002/heygpt/releases/download/latest-macOS-X64/heygpt-macOS-X64.tar.gz
    tar -xzf heygpt-macOS-X64.tar.gz
elif [[ "$OSTYPE" == "darwin"* && "$_ARCH" == "arm64"  ]]; then
    wget https://github.com/p208p2002/heygpt/releases/download/latest-macOS-ARM64/heygpt-macOS-ARM64.tar.gz
    tar -xzf heygpt-macOS-ARM64.tar.gz
else
    echo "Heygpt not available for this computer"
fi


cp target/release/heygpt /usr/local/bin

rm -rf /tmp/target