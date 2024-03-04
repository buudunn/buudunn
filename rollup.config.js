import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        index: "Cargo.toml",
    },
    output: {
      dir: "output",
      format: "es" 
    },
    plugins: [
        rust({
          inlineWasm: true,
          nodejs: false
        }),
    ],
};