
import React, { useMemo, useState } from "react";
import { useDndMonitor, DragEndEvent} from '@dnd-kit/core';

import { WasmPos } from "./wasm";
import { DroppableSquare } from "./Square";
import { JsPos, JsState } from "./types";
import { Piece, DraggablePiece } from "./Piece";
import { splitMv } from "./util";

import './Board.css';

interface BoardProps {
  state: JsState;
  mkMove: (a: JsPos, b: JsPos) => void;
  canMove: boolean;
  flipped?: boolean;
}
export default function Board({state, mkMove, canMove, flipped}: BoardProps) {
  const {grid, mvMap} = useMemo(() => {
    const grid = state.st.boardString().split('\n').reverse().map(r => r.split(' '));
    const moves = state.st.moveGen().split(' ').map(splitMv);
    const mvMap = moves.reduce((map, [a, b]) => {
      if (!map.get(a)) map.set(a, new Set());
      map.get(a)!.add(b);
      return map;
    }, new Map<JsPos, Set<JsPos>>());
    return {grid, mvMap};
  }, [state]);
  
  const bw = grid.length, bh = grid[0].length;
  const [selected, setSelected] = useState<JsPos>();
  const validDest = (b?: JsPos) => !!selected && !!b && mvMap.get(selected)?.has(b);
  function onDragCancel() {
    setSelected(undefined);
  }
  function onDragEnd(event: DragEndEvent) {
    const over = event.over?.id;
    if (canMove && validDest(over)) mkMove(selected!, over!);
    onDragCancel();
  }
  useDndMonitor({
    onDragStart(event) {
      setSelected(event.active.id);
    },
    onDragEnd,
    onDragCancel
  });

  const ys = [...Array(bh)].map((_,i) => i);
  const xs = [...Array(bw)].map((_,j) => j);
  if (!flipped) ys.reverse();

  return (
    <div className="board-wrap css-sq">
      <div className="board" role="grid" style={{
        gridTemplateColumns: `repeat(${bw}, 1fr)`,
        gridTemplateRows: `repeat(${bh}, 1fr)`
      }}>
        {ys.flatMap(y => <div key={y} role="row" className="d-contents">
          {xs.map(x => {
            const pc = grid[y][x];
            const pos: JsPos = `${new WasmPos(y, x)}`;
            const dark = (y+x) % 2 === 0;
            const Pc = mvMap.has(pos) ? DraggablePiece : Piece;
            const hasPc = pc !== '.';
            const piece = hasPc && <Pc {...{pos, pc}}/>;

            const isSrc = selected === pos;
            const isDest = validDest(pos);
            const dropClass = hasPc ? 'frame' : 'circ';
            const bg = isSrc ? 'cover' :
              isDest ? dropClass :
                undefined;
            const desc = pos;
            return <DroppableSquare key={pos} {...{desc, pos, dark, bg}}>
              {piece}
            </DroppableSquare>;
          })}</div>)}
      </div>
    </div>);
}