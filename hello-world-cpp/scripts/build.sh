#!/bin/bash

set -e

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts" ]; then
  cd ..
fi

if [ ! -d ic-wasi-polyfill ]; then
  git clone https://github.com/wasm-forge/ic-wasi-polyfill.git
  cd ic-wasi-polyfill
  cargo build --release --target wasm32-wasip1
  cd ..
fi

[ -d target ] || mkdir target

/opt/wasi-sdk/bin/clang++ -mexec-model=reactor -fno-exceptions src/main.cpp -L./ic-wasi-polyfill/target/wasm32-wasip1/release -lic_wasi_polyfill -o target/main.wasm

ic-wasm target/main.wasm -o target/meta.wasm metadata candid:service -f ./src/hello-cpp.did -v public

wasi2ic target/meta.wasm target/nowasi.wasm

popd
