#!/bin/bash

apt update
apt install -y \
    apt-utils \
    build-essential \
    curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
echo "export PATH=\"$HOME/.cargo/bin:$PATH\"" >>~/.bashrc
