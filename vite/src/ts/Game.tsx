import React, { useCallback, useEffect, useMemo, useState } from 'react';
import {WasmState} from './wasm';
import Board from './Board';
import { DndContext } from '@dnd-kit/core';
import { GameBot, JsPos, JsState, Phase } from './types';
import { negamax, splitMv } from './util';

interface GameProps {
  setPhase: React.Dispatch<Phase>;
  bot?: GameBot;
}
export default function Game({bot}: GameProps) {
  const [state, setState] = useState(() => new JsState(new WasmState()));
  const isWhite = useMemo(() => state.st.isWhite(), [state]);
  
  const flipped = bot ? bot.isWhite : !isWhite;
  // look at the declaration of JsState to understand this nonsense
  const mkMove = useCallback((a: JsPos, b: JsPos) => setState(
    state => state.mut(st => st.makeMove(a+b))
  ), [setState]);
  const canMove = bot ? bot.isWhite !== isWhite : true;
  useEffect(() => {
    if (canMove || !bot) return;
    let finished = false;
    const {promise, cancel} = negamax({
      fen: `${state.st}`,
      depth: bot.depth
    });
    promise.then(({mv}) => {
      finished = true;
      if (mv) mkMove(...splitMv(mv))
    });
    return () => {
      if (!finished) cancel();
    };
  }, [isWhite, bot, canMove]);

  return (<DndContext>
    <Board {...{state, mkMove, canMove, flipped}} />
  </DndContext>);
}
