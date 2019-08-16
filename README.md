# `wasm-bindings` NPM module

This directory contains the `wasm-bindings` NPM module. This NPM module is
written in Rust and compiled to WebAssembly using `wasm-pack`. The purpose of
this package is to provide a polyfill for [WebAssembly
bindings](https://github.com/webassembly/webidl-bindings) usable in JS.

This module is consumed by the Node and Web examples of this repository.

If you're curious about how this is implemented, it:

* Uses `wasm-pack`
* ... to use the `wasm-bindgen` CLI tool
* ... to export a Rust crate to JS by compiling to WebAssembly
* ... which links to the `wasm-bindgen-cli-support` library that `wasm-bindgen`
  itself uses

In other words this NPM module uses `wasm-bindgen` to compile `wasm-bindgen` to
consume `wasm-bindgen`-compiled modules. Yo dawg...
