import init, { main } from '/pkg/budunn.js';
import sample from '/pkg/buudunn_bg.wasm';

sample()
  .then({ instance }, init(instance))
  .then(() => main());

// or using top-level await

await init(await sample());
main();