#!/bin/bash

if ! command -v pre-commit &>/dev/null; then
    echo "command not found"
    ./scripts/install_pre-commit.sh
fi
pre-commit run --all-files
