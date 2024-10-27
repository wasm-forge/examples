#!/bin/bash

dfx canister create hello-world-backend

set -e

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts" ]; then
  cd ..
fi

cargo build --release --target wasm32-wasip1 

wasi2ic ./target/wasm32-wasip1/release/hello_world_backend.wasm  ./target/wasm32-wasip1/release/no_wasi.wasm

dfx canister install --mode reinstall -y --wasm ./target/wasm32-wasip1/release/no_wasi.wasm hello-world-backend

popd
