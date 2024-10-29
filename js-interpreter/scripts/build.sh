#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-wasip1/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts" ]; then
  cd ..
fi

cargo build --release --target wasm32-wasip1

ic-wasm $RELEASE_DIR/js_interpreter_backend.wasm -o $RELEASE_DIR/meta.wasm metadata candid:service -f ./src/js-interpreter-backend/js-interpreter-backend.did -v public

wasi2ic $RELEASE_DIR/meta.wasm $RELEASE_DIR/no_wasi.wasm

popd