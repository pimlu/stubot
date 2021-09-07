import { useDroppable } from "@dnd-kit/core";
import React from "react";
import { JsPos } from "./types";

interface SquareBase {
  dark: boolean;
  bg?: 'cover' | 'circ' | 'frame';
  pos: JsPos;
  desc?: string;
  children: React.ReactNode;
}
type SquareProps = SquareBase;

export const Square = React.memo(React.forwardRef<HTMLDivElement, SquareProps>(
  function Square({dark, bg, desc, children}: SquareProps, ref) {
    return <div
      ref={ref}
      role="gridcell"
      className={`sq css-sq ${dark ? 'db' : 'lb'}`}>
      <div className={`sq-inner fit-abs ${bg ? bg : ''}`} 
        aria-label={desc}>
        {children}
      </div>
    </div>;
  }));

type DroppableSquareProps = SquareBase;

export function DroppableSquare({pos, bg, ...rest}: DroppableSquareProps) {
  const {setNodeRef} = useDroppable({
    id: pos,
  });
  return <Square ref={setNodeRef} {...{pos, bg, ...rest}} />
}