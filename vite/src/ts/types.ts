import { WasmState } from "./wasm";

// wrap in an obj to trick rerenders. WasmState will be === even after mut
export class JsState {
  st: WasmState;
  constructor(st: WasmState) {
    this.st = st;
  }
  mut(fn: (wst: WasmState) => void) {
    fn (this.st);
    return new JsState(this.st);
  }
}
export type JsPos = string;
