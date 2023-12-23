#!/bin/bash

cargo build --target wasm32-unknown-emscripten --release
mkdir -p lib
cp target/wasm32-unknown-emscripten/release/mozjpeg-wasm.js lib/mozjpeg-wasm.js
cp target/wasm32-unknown-emscripten/release/mozjpeg_wasm.wasm lib/mozjpeg_wasm.wasm
