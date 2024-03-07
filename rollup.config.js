import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        index: "index.js",
    },
    output: {
      dir: "output",
      format: "iife" 
    },
    plugins: [
        rust({
          inlineWasm: true,
          nodejs: false
        }),
    ],
};