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
    pub mv: Move,
    pub score: i16,
}

impl State {
    fn add_sudo(&mut self, moves: &mut Vec<MvMeta>, mv: Move) {
        self.make_move(mv);
        moves.push(MvMeta { mv, score: 0 });
        self.unmake_move();
    }
    // pushes the move if there is space, returns whether ray should cont.
    // except if gate(Sq) is false, it just stops early.
    fn try_move(
        &mut self,
        gate: impl Fn(bool) -> bool,
        moves: &mut Vec<MvMeta>,
        orig: Pos,
        pos: Pos,
    ) -> bool {
        let mut mv = Move {
            a: orig,
            b: pos,
            capture: None,
            extra: None,
        };
        let pos_sq = match self.get(pos) {
            Some(sq) => *sq,
            None => return false,
        };
        if !gate(pos_sq.0.is_some()) {
            return false;
        }

        let Piece { clr, typ } = match pos_sq {
            Sq(Some(pc)) => pc,
            Sq(None) => {
                self.add_sudo(moves, mv);
                return true;
            }
        };
        if clr != self.turn() {
            mv.capture = Some(typ);
            self.add_sudo(moves, mv);
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
                    self.try_move(|take| take == $is_take, &mut moves, orig, orig + $dir)
                };
            }
            macro_rules! try_move {
                () => {
                    |pos| self.try_move(|_| true, &mut moves, orig, pos)
                };
            }
            let Piece { clr, typ } = match self.get(orig).unwrap() {
                Sq(Some(pc)) => *pc,
                Sq(None) => return,
            };
            if clr != self.turn() {
                return;
            }

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
                Type::Knight => leaper(orig, KNIGHT_OPTS, try_move!()),
                Type::Bishop => rider(orig, BISHOP_OPTS, try_move!()),
                Type::Rook => rider(orig, ROOK_OPTS, try_move!()),
                Type::Queen => {
                    rider(orig, BISHOP_OPTS, try_move!());
                    rider(orig, ROOK_OPTS, try_move!());
                }
                Type::King => {
                    leaper(orig, KING_OPTS, try_move!());
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
