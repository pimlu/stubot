import { useDroppable } from "@dnd-kit/core";
import React from "react";
import { JsPos } from "./types";

interface SquareBase {
  dark: boolean;
  bg?: 'cover' | 'circ' | 'frame';
  children: React.ReactNode;
}
type SquareProps = SquareBase;

export const Square = React.memo(React.forwardRef<HTMLDivElement, SquareProps>(
  function Square({dark, bg, children}: SquareProps, ref) {
    return <div
      ref={ref}
      className={`sq ${dark ? 'db' : 'lb'}`}>
      <div className={`sq-inner center fit ${bg ? bg : ''}`}>
        {children}
      </div>
    </div>;
  }));

interface DroppableSquareProps extends SquareBase {
  pos: JsPos;
}

export function DroppableSquare({dark, pos, bg: bg_, children}: DroppableSquareProps) {
  const {isOver, setNodeRef} = useDroppable({
    id: pos,
  });
  const bg = isOver ? 'cover' : bg_;
  return <Square ref={setNodeRef} {...{dark, bg, children}} />
}