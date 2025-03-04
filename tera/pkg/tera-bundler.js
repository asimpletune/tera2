import * as imports from "./tera-bundler_bg.js";

// switch between both syntax for node and for workerd
import wkmod from "./tera-bundler_bg.wasm";
import * as nodemod from "./tera-bundler_bg.wasm";
if (typeof process !== "undefined" && process.release.name === "node") {
  imports.__wbg_set_wasm(nodemod);
} else {
  const instance = new WebAssembly.Instance(wkmod, {
    "./tera-bundler_bg.js": imports,
  });
  imports.__wbg_set_wasm(instance.exports);
}

export * from "./tera-bundler_bg.js";