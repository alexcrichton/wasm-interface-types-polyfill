# `wasm-interface-types` NPM module

This repository contains the `wasm-interface-types` NPM module. This NPM module
is written in Rust and compiled to WebAssembly using [`wasm-pack`](curl
https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f). The
purpose of this package is to provide a polyfill for [WebAssembly
Interface Types](https://github.com/webassembly/webidl-bindings) to be usable in
JS.

## Building the module

The module can be built with

```
$ wasm-pack build
```

and the `pkg` directory will have the NPM module to publish

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
