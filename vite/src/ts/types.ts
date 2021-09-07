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
export interface GameBot {
  depth: number;
  isWhite: boolean;
}
interface IntroPhase {
  cur: 'intro';
}
export interface GamePhase {
  cur: 'game';
  bot?: GameBot;
}
export type Phase = IntroPhase | GamePhase;

export interface AiQuery {
  fen: string;
  depth: number;
}
export interface AiResponse {
  score: number;
  mv?: string;
  nodes: number;
}