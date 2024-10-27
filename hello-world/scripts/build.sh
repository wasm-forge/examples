#!/bin/bash

export RELEASE_DIR=./target/wasm32-wasip1/release
set -e

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts" ]; then
  cd ..
fi

cargo build --release --target wasm32-wasip1

ic-wasm $RELEASE_DIR/hello_world_backend.wasm -o $RELEASE_DIR/meta.wasm metadata candid:service -f ./src/hello-world-backend/hello-world-backend.did -v public

wasi2ic $RELEASE_DIR/meta.wasm $RELEASE_DIR/no_wasi.wasm

popd
