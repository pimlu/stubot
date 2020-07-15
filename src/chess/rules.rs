use super::card;
use super::*;
use crate::types;

struct BoardHistory {
    history: Vec<State>,
}

fn rider(orig: Pos, options: &'static [Pos], mut f: impl FnMut(Pos) -> bool) {
    for dir in options {
        let mut pos = orig + *dir;
        while f(pos) {
            pos += *dir;
        }
    }
}
fn leaper(orig: Pos, options: &'static [Pos], mut f: impl FnMut(Pos) -> bool) {
    for dir in options {
        f(orig + *dir);
    }
}

const KNIGHT_OPTS: &[Pos] = &[
    Pos { x: 1, y: 2 },
    Pos { x: -1, y: 2 },
    Pos { x: 1, y: -2 },
    Pos { x: -1, y: -2 },
    Pos { x: 2, y: 1 },
    Pos { x: -2, y: 1 },
    Pos { x: 2, y: -1 },
    Pos { x: -2, y: -1 },
];

const BISHOP_OPTS: &[Pos] = &[
    Pos { x: 1, y: 1 },
    Pos { x: 1, y: -1 },
    Pos { x: -1, y: 1 },
    Pos { x: -1, y: -1 },
];
const ROOK_OPTS: &[Pos] = &[
    Pos { x: 1, y: 0 },
    Pos { x: -1, y: 0 },
    Pos { x: 0, y: 1 },
    Pos { x: 0, y: -1 },
];

impl types::MoveGen for State {
    type Move = Move;
    fn get_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::<Move>::new();
        // pushes the move if there is space, returns whether ray should cont.
        let try_move = |moves: &mut Vec<Move>, orig, pos| {
            let mv = Move { a: orig, b: pos };
            let Piece { c, t: _ } = match self.get(pos) {
                Some(Sq(Some(p))) => p,
                Some(Sq(None)) => {
                    moves.push(mv);
                    return true;
                }
                None => return false,
            };
            if *c != self.turn {
                moves.push(mv);
            }
            return false;
        };
        // take or move-only moves for pawns. returns whether it could move
        let special_move = |moves: &mut Vec<Move>, orig, pos, is_take: bool| {
            let mv = Move { a: orig, b: pos };
            let Piece { c, t } = match self.get(pos) {
                Some(Sq(Some(p))) => p,
                Some(Sq(None)) => {
                    if !is_take {
                        moves.push(mv);
                    }
                    return !is_take;
                }
                None => return false,
            };
            if *c != self.turn && is_take {
                moves.push(mv);
                return true;
            }
            return false;
        };

        let mut add_moves = |orig| {
            macro_rules! special_move {
                ($dir:expr, $is_take:expr) => {
                    special_move(&mut moves, orig, orig + $dir, $is_take);
                };
            }
            let Piece { c, t } = match self.get(orig).unwrap() {
                Sq(Some(p)) => p,
                Sq(None) => return,
            };
            if *c != self.turn {
                return;
            }
            let try_m = |pos| try_move(&mut moves, orig, pos);

            match *t {
                Type::Pawn => {
                    let home_row = orig.y
                        == match *c {
                            Color::White => 1,
                            Color::Black => 6,
                        };
                    let dir = match *c {
                        Color::White => card::N,
                        Color::Black => card::S,
                    };
                    // move-onlies
                    let first_push = special_move!(dir, false);
                    if first_push && home_row {
                        special_move!(dir + dir, false);
                    }
                    // diagonal
                    special_move!(dir + card::W, true);
                    special_move!(dir + card::E, true);
                }
                Type::Knight => leaper(orig, KNIGHT_OPTS, try_m),
                Type::Bishop => rider(orig, BISHOP_OPTS, try_m),
                Type::Rook => rider(orig, ROOK_OPTS, try_m),
                Type::Queen => {
                    rider(orig, BISHOP_OPTS, try_m);
                    rider(orig, ROOK_OPTS, try_m);
                }
                Type::King => {
                    leaper(orig, BISHOP_OPTS, try_m);
                    leaper(orig, ROOK_OPTS, try_m);
                }
            }
        };
        for y in 0..BOARD_DIM.y {
            for x in 0..BOARD_DIM.x {
                add_moves(Pos { y, x });
            }
        }
        moves
    }
}
