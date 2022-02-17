#!/bin/bash

cargo fmt \
    --check \
    --all \
    --verbose \
    --manifest-path=Cargo.toml \
    --message-format=human