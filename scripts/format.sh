#!/bin/bash

./scripts/install_rustfmt.sh

cargo fmt \
    --check \
    --all \
    --verbose \
    --manifest-path=Cargo.toml \
    --message-format=human
<<<<<<< HEAD

./scripts/pre-commit.sh
=======
>>>>>>> f7698da3816fb237c30d60643092a1f2d76fff4c
