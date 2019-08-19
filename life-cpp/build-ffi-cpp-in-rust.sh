#!/bin/bash

set -xeuo pipefail

CPP_ROOT=$(cd "$(dirname $0)"; pwd)
PROJECT_ROOT="$CPP_ROOT/.."

cd "$CPP_ROOT"
rm -rf build-ffi-export
mkdir build-ffi-export
cd build-ffi-export
cmake "$CPP_ROOT"
make

pushd "$PROJECT_ROOT/life-rs"

CPP_DIR="$CPP_ROOT/build-ffi-export" cargo run --example glider --features ffi_import
popd
