import wasmInit from '../../../stubot-wasm/pkg/stubot_wasm';
import wasm from '../../../stubot-wasm/pkg/stubot_wasm_bg.wasm?url';
export default function init() {
  let path = wasm;
  // in web worker, URLs are messed up
  if (self.document === undefined) {
    path = path.replace('./assets', '.');
  }
  return wasmInit(new URL(path, `${self.location}`));
}

export * from '../../../stubot-wasm/pkg/stubot_wasm';
