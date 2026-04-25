#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-wasip1/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts" ]; then
  cd ..
fi

cargo build --release --target wasm32-wasip1
wasi2ic $RELEASE_DIR/hello_world_backend.wasm $RELEASE_DIR/no_wasi.wasm

popd
