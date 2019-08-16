# `wasm-interface-types-loader` NPM module

This directory contains a Webpack loader called `wasm-interface-types-loader`
used for loading a `*.wasm` file directly with Webpack where the `*.wasm` file
has a custom section for [WebAssembly Interface
Types](https://github.com/webassembly/webidl-bindings).

## Example usage

First, you'll need to install this package with:

```
$ npm intsall wasm-interface-types-loader
```

Next you'll want to add this to your `webpack.config.js`:

```js
module.exports = {
  // ...
  module: {
    rules: [
      {
        test: /^(?:(?!_bg).)*\.wasm$/i,
        type: "javascript/auto",
        use: "wasm-interface-types-loader",
      },
    ],
  },
  // ...
};
```

And then in your code you can use:

```js
import { myFunction } from "./my-wasm-file.wasm";
```

Or if you're in the "main chunk" you'll have to use a dynamic import:

```js
async function run() {
  const { myFunction } = await import("./my-wasm-file.wasm");
}

run();
```

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
