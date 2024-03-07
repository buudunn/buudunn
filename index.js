import wasm from './Cargo.toml'
window.onerror = (event, source, lineno, colno, error) => window.alert(`${event} in ${source} at ${lineno}:${colno}; ${error}`);

async function loadWasm() {
  const exports = wasm;
  const wbg = await exports();

  await wbg.start();
  let context = document.getElementById('canvas').getContext('2d');
  window.addEventListener("keydown", async (event) => await wbg.keydownhandler(event, context));
}

loadWasm();