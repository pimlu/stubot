use super::*;
use crate::types;

struct BoardHistory {
    history: Vec<State>,
}

enum Fairy {
    Leaper(&'static [Pos]),
    Rider(&'static [Pos]),
}

fn rider(orig: Pos, dir: Pos, mut f: impl FnMut(Pos) -> bool) {
    let Pos { mut y, mut x } = orig;
    let Pos { y: dy, x: dx } = dir;
    loop {
        y += dy;
        x += dx;
        if !f(Pos { y, x }) {
            break;
        }
    }
}

impl types::MoveGen for State {
    type Move = Move;
    fn get_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::<Move>::new();

        let mut add_moves = |pos| {
            let Piece { c, t } = match self.get(pos).unwrap() {
                Sq(Some(p)) => p,
                Sq(None) => return,
            };
            //TODO
            moves.push(Move { a: pos, b: pos });
        };
        for y in 0..BOARD_DIM.y {
            for x in 0..BOARD_DIM.x {
                add_moves(Pos { y, x });
            }
        }
        moves
    }
}
