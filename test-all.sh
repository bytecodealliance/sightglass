#!/bin/bash
set -euo pipefail

function banner {
    echo "======  $*  ======"
}

# sightglass
banner "Test sightglass"
cargo test

# sg-history
banner "Test sg-history"
pushd webui/sg-history
cargo test
popd

# sg-ui
banner "Test sg-view"
pushd webui/sg-view
yarn install
npm test
popd
