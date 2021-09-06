import wasmInit from '../../../stubot-wasm/pkg/stubot_wasm';
import wasm from '../../../stubot-wasm/pkg/stubot_wasm_bg.wasm?url';
export default function init() {
  return wasmInit(new URL(wasm, `${self.location}`));
}

export * from '../../../stubot-wasm/pkg/stubot_wasm';
