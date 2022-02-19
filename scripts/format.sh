#!/bin/bash

./scripts/install_nightly.sh
./scripts/install_rustfmt.sh

cargo fmt \
    --all \
    --verbose \
    --manifest-path=Cargo.toml \
    --message-format=human
# --check

./scripts/pre-commit.sh
