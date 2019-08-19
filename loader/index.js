const fs = require('fs');
const path = require('path');
const util = require('util');
const wasm_interface_types = require("wasm-interface-types");

const writeFileAsync = util.promisify(fs.writeFile);

async function run(cx, source) {
  const name = path.basename(cx.resourcePath, '.wasm');
  if (name.indexOf('_bg') !== -1)
    return source;
  const ret = await wasm_interface_types.compile(name, source, true);
  // TODO: should figure out how to not need to write to the actual filesystem
  // here.
  await writeFileAsync(path.resolve(path.dirname(cx.resourcePath), name + '_bg.wasm'), ret.wasm());
  return ret.js();
}

module.exports = function (source, a, b) {
  var callback = this.async();

  run(this, source)
    .then((x) => {
      callback(null, x);
    })
    .catch((e) => {
      e.stack = "";
      callback(e);
    });
};
module.exports.raw = true;
