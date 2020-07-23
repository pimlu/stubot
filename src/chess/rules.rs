use super::card;
use super::*;

// no trait aliases yet
trait PosCb: FnMut(Pos) -> bool {}
trait MoveCb: FnMut(Move) {}

impl<T: FnMut(Pos) -> bool> PosCb for T {}
impl<T: FnMut(Move)> MoveCb for T {}

fn rider(orig: Pos, options: &'static [Pos], mut f: impl PosCb) {
    for dir in options {
        let mut pos = orig + *dir;
        while f(pos) {
            pos += *dir;
        }
    }
}
fn leaper(orig: Pos, options: &'static [Pos], mut f: impl PosCb) {
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
// Everything but left and right (which are special)
const KING_OPTS: &[Pos] = &[
    Pos { x: 1, y: 1 },
    Pos { x: 1, y: -1 },
    Pos { x: -1, y: 1 },
    Pos { x: -1, y: -1 },
    Pos { x: 0, y: 1 },
    Pos { x: 0, y: -1 },
];
pub struct MvMeta {
    mv: Move,
    p: Piece,
    cap: Option<Type>,
    score: i16,
}

impl State {
    fn add_sudo(&mut self, moves: &mut Vec<MvMeta>, mv: Move) {
        self.make_move(mv);

        self.unmake_move();
    }
    // pushes the move if there is space, returns whether ray should cont.
    fn try_move(&mut self, moves: &mut Vec<MvMeta>, orig: Pos, pos: Pos) -> bool {
        let mv = Move {
            a: orig,
            b: pos,
            extra: None,
        };
        let Piece { clr, typ: _ } = match self.get(pos) {
            Some(Sq(Some(pc))) => *pc,
            Some(Sq(None)) => {
                self.add_sudo(moves, mv);
                return true;
            }
            None => return false,
        };
        if clr != self.turn() {
            self.add_sudo(moves, mv);
        }
        return false;
    }

    // take or move-only moves for pawns. returns whether it could move
    fn special_move(
        &mut self,
        moves: &mut Vec<MvMeta>,
        orig: Pos,
        pos: Pos,
        is_take: bool,
    ) -> bool {
        let mv = Move {
            a: orig,
            b: pos,
            extra: None,
        };
        let Piece { clr, typ: _ } = match self.get(pos) {
            Some(Sq(Some(pc))) => *pc,
            Some(Sq(None)) => {
                if !is_take {
                    self.add_sudo(moves, mv);
                }
                return !is_take;
            }
            None => return false,
        };
        if clr != self.turn() && is_take {
            self.add_sudo(moves, mv);
            return true;
        }
        return false;
    }

    // requires a mutable reference, but doesn't actually modify anything
    // (if our code is correct)
    pub fn get_moves(&mut self) -> Vec<MvMeta> {
        let mut moves = Vec::<MvMeta>::new();
        let mut add_moves = |orig| {
            macro_rules! special_move {
                ($dir:expr, $is_take:expr) => {
                    self.special_move(&mut moves, orig, orig + $dir, $is_take);
                };
            }
            let Piece { clr, typ } = match self.get(orig).unwrap() {
                Sq(Some(pc)) => *pc,
                Sq(None) => return,
            };
            if clr != self.turn() {
                return;
            }
            let try_m = |pos| false; //self.try_move(&mut moves, orig, pos);

            match typ {
                Type::Pawn => {
                    let home_row = orig.y
                        == match clr {
                            Color::White => 1,
                            Color::Black => 6,
                        };
                    let dir = match clr {
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
                    //rider(orig, ROOK_OPTS, try_m);
                }
                Type::King => {
                    leaper(orig, KING_OPTS, try_m);
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
