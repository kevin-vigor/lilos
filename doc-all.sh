#!/usr/bin/env bash

set -euo pipefail

DIRS="os extra/*"

for d in $DIRS; do
    echo "---- testing doc generation in $d"
    pushd $d > /dev/null
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
    popd > /dev/null
done
