#!/bin/bash

set -xeuo pipefail

CPP_ROOT=$(cd "$(dirname $0)"; pwd)
PROJECT_ROOT="$CPP_ROOT/.."
pushd "$PROJECT_ROOT/life-rs"

cargo build --features ffi_export
popd

rm -rf build-ffi-import
mkdir build-ffi-import
cd build-ffi-import
cmake "$CPP_ROOT" -DFFI_DIR="$PROJECT_ROOT/target/debug"
make
./life-cpp
