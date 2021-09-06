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
      className={`sq css-sq ${dark ? 'db' : 'lb'}`}>
      <div className={`sq-inner fit-abs ${bg ? bg : ''}`}>
        {children}
      </div>
    </div>;
  }));

interface DroppableSquareProps extends SquareBase {
  pos: JsPos;
}

export function DroppableSquare({pos, bg, ...rest}: DroppableSquareProps) {
  const {setNodeRef} = useDroppable({
    id: pos,
  });
  return <Square ref={setNodeRef} {...{bg, ...rest}} />
}