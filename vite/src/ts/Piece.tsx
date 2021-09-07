import React from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faChessPawn, faChessKnight, faChessBishop,
  faChessRook, faChessQueen, faChessKing, IconDefinition
} from '@fortawesome/free-solid-svg-icons';
import { useDraggable } from '@dnd-kit/core';
import { JsPos } from './types';



const pieceMap: Record<string, IconDefinition> = {
  'P': faChessPawn,
  'N': faChessKnight,
  'B': faChessBishop,
  'R': faChessRook,
  'Q': faChessQueen,
  'K': faChessKing
};

interface PieceBase {
  pc: string,
}
interface PieceProps extends PieceBase, React.HTMLAttributes<HTMLElement> {
}
export const Piece = React.memo(React.forwardRef<HTMLDivElement, PieceProps>(
  function Piece({pc, ...rest}, ref) {
    const upper = pc.toUpperCase();
    const isWhite = pc === upper;
    const icon = pieceMap[upper];

    return <div className="pc-frame center fit-pct" ref={ref} {...rest}>
      <div className="pc">
        <FontAwesomeIcon
          icon={icon}
          className={`fit-pct ${isWhite ? 'w' : 'b'}`}/>
      </div>
    </div>;
  }));

interface DraggablePieceProps extends PieceBase {
  pos: JsPos;
}
export function DraggablePiece({pos, pc}: DraggablePieceProps) {
  const {attributes, listeners, setNodeRef, transform, isDragging} = useDraggable({
    id: pos,
  });

  const style = transform ? {
    transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
    zIndex: isDragging ? 2 : undefined
  } : undefined;
  return <Piece ref={setNodeRef} pc={pc} id={`pc-${pos}`} style={style} {...listeners} {...attributes} />
}