import React, { useCallback, useMemo, useState } from 'react';
import {WasmState} from './wasm';
import Board from './Board';
import { DndContext } from '@dnd-kit/core';
import { JsPos, JsState } from './types';


export default function App() {
  const st = useMemo(() => new JsState(new WasmState()), []);
  const [state, setState] = useState(st);
  const flipped = false;
  // look at the declaration of JsState to understand this nonsense
  const move = useCallback((a: JsPos, b: JsPos) => setState(
    state => state.mut(st => st.makeMove(a+b))
  ), [setState]);
  return (<DndContext>
    <Board {...{state, move, flipped}} />
  </DndContext>);
}
