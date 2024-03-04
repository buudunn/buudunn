async function loadWasm() {
  const wasm = await import("./path/to/Cargo.toml");
  const exports = await wasm.default();

  // Use functions which were exported from Rust...
}