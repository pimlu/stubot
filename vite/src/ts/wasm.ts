import wasmInit from '../../../stubot-wasm/pkg/stubot_wasm';
import wasm from '../../../stubot-wasm/pkg/stubot_wasm_bg.wasm?url';
type Cb = (u: unknown) => void;
let res: Cb, rej: Cb;
// dumb variable because we can't synchronously poll promises
export let isInitDone = false;
export const initDone = new Promise((res_, rej_) => {
  res = res_;
  rej = rej_;
});
async function init_() {
  let path = wasm;
  // in web worker, URLs are messed up
  if (self.document === undefined) {
    path = path.replace('./assets', '.');
  }
  await wasmInit(new URL(path, `${self.location}`));
  isInitDone = true;
}
export default function init() {
  init_().then(res).catch(rej);
  return initDone;
}

export * from '../../../stubot-wasm/pkg/stubot_wasm';
