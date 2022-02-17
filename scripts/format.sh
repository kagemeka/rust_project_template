#!/bin/bash

./scripts/install_rustfmt.sh

cargo fmt \
    --check \
    --all \
    --verbose \
    --manifest-path=Cargo.toml \
    --message-format=human

./scripts/pre-commit.sh
