import init, {WasmSearcher, WasmPos} from '../../../stubot-wasm/pkg/stubot_wasm';
init();

(window as any).WasmSearcher = WasmSearcher;
(window as any).WasmPos = WasmPos;

export function App() {
  return (
    <p>chess app</p>
  )
}
