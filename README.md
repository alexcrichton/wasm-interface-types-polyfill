# `wasm-interface-types` NPM module

This repository contains the `wasm-interface-types` NPM module. This NPM module
is written in Rust and compiled to WebAssembly using [`wasm-pack`](curl
https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f). The
purpose of this package is to provide a polyfill for [WebAssembly
Interface Types](https://github.com/webassembly/webidl-bindings) to be usable in
JS.

## Using this module

A pre-built version of this module can be installed with

```
$ npm install wasm-interface-types
```

This module is intended to be consumed by Node.js. Versions that run in a
browser are not compiled yet.

Example usage looks like:

```js
const wit = require('wasm-interface-types');

async function run(wasmBytes) {
    // You can either execute the module directly in Node.js...
    const module = await wit.process(wasmBytes, /* esm =*/ false);
    module.exported_function();

    // ... or you can compile it and inspect the JS/wasm
    const result = await wit.compile(wasmBytes, /* esm =*/ true);
    console.log(result.js());
    console.log(result.wasm());
}

const wasmBytes = [ /* ... */ ];
run(wasmBytes);
```

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
