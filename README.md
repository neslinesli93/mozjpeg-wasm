# mozjpeg-wasm

![npm (scoped)](https://img.shields.io/npm/v/@neslinesli93/mozjpeg-wasm)

This library wraps [`mozjpeg-sys`](https://github.com/kornelski/mozjpeg-sys) and exposes a few functions to perform decoding, encoding and simple transformation on JPEG images using [`mozjpeg`](https://github.com/mozilla/mozjpeg).

Everything is compiled to WebAssembly and bundled in an NPM package which can be used directly in a browser.

## Install

```sh
$ npm i @neslinesli93/mozjpeg-wasm
```

## Usage

The library contains two files:

- `mozjpeg-wasm.wasm`: it's the WebAssembly bundle that exposes all the functions to perform operations on JPEG images
- `mozjpeg-wasm.js`: it exposes JS glue code that can be used to call the functions defined in the WebAssembly module

An example on the usage can be seen inside the [JPEG.rocks repo](https://github.com/neslinesli93/jpeg.rocks/tree/master/src/converter)

## Build from source

Prerequisites:

1. Emscripten (>= `1.39.20`)
2. Rust (>= `1.48.0`)
3. Rust target `wasm32-unknown-emscripten`

Then you can use the following script which builds the crate using `wasm32-unknown-emscripten` target and move the resulting js/wasm files to `lib`:

```sh
$ ./build.sh
```
