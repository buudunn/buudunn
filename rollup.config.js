import { wasm } from '@rollup/plugin-wasm';

export default {
  input: 'index.js',
  output: {
    dir: 'output',
    format: 'es',
  },
  plugins: [wasm({
    targetEnv: "auto-inline"
  })]
};