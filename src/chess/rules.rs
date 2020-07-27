use super::card;
use super::*;

use std::cmp;

fn pawn_dir(clr: Color) -> Pos {
    match clr {
        Color::White => card::N,
        Color::Black => card::S,
    }
}
// no trait aliases yet
trait PosCb: FnMut(Pos) -> bool {}
impl<T: FnMut(Pos) -> bool> PosCb for T {}

fn rider(orig: Pos, options: &'static [Pos], mut f: impl PosCb) {
    for &dir in options {
        let mut pos = orig + dir;
        while f(pos) {
            pos += dir;
        }
    }
}
fn leaper(orig: Pos, options: &'static [Pos], mut f: impl PosCb) {
    for &dir in options {
        f(orig + dir);
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MvMeta {
    pub mv: Move,
    pub score: i16,
}

impl State {
    fn add_sudo(&mut self, moves: &mut Vec<MvMeta>, mv: Move) {
        let clr = self.turn();
        self.make_move(mv);
        let king_pos = *self.get_king_pos(clr);
        // make sure there is actually a king where we are guarding
        // for check
        debug_assert!(*self.idx(king_pos) == Sq::new(clr, Type::King));
        if !self.is_attacked(king_pos, clr.other()) {
            moves.push(MvMeta { mv, score: 0 });
        }
        self.unmake_move();
    }
    fn is_attacked(&self, orig: Pos, enemy: Color) -> bool {
        use std::cell::Cell;

        let enemy_knight = Sq::new(enemy, Type::Knight);
        for &opt in KNIGHT_OPTS {
            if self.get(orig + opt) == Some(&enemy_knight) {
                return true;
            }
        }
        let pdir = pawn_dir(enemy);
        let enemy_pawn = Sq::new(enemy, Type::Pawn);
        for &side in &[card::E, card::W] {
            if self.get(orig + pdir + side) == Some(&enemy_pawn) {
                return true;
            }
        }
        let found_attack = &Cell::new(false);
        let attack = || {
            found_attack.set(true);
        };
        let check = |threat| {
            move |pos| {
                let Piece { clr, typ } = match self.get(pos) {
                    Some(Sq(Some(pc))) => *pc,
                    // keep scanning if we haven't found a check
                    Some(Sq(None)) => return !found_attack.get(),
                    None => return false,
                };

                if clr == enemy {
                    if typ == threat || typ == Type::Queen {
                        attack();
                    } else if typ == Type::King {
                        let diff = pos - orig;
                        // king should be right next to them
                        if cmp::max(diff.x.abs(), diff.y.abs()) <= 1 {
                            attack();
                        }
                    }
                }
                // scan threats bump into pieces
                false
            }
        };
        rider(orig, BISHOP_OPTS, check(Type::Bishop));
        rider(orig, ROOK_OPTS, check(Type::Rook));

        found_attack.get()
    }
    // pushes the move if there is space, returns whether ray should continue.
    // return value is unused in some cases.
    // except if gate(Sq) is false, it just stops early. gate is unused (just
    // returns true) in some cases.
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
                ($pos: expr) => {
                    try_move!()($pos)
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
                    let dir = pawn_dir(clr);
                    // move-onlies
                    let first_push = special_move!(dir, false);
                    if first_push && home_row {
                        special_move!(dir * 2, false);
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
                    // first add the left/right moves. if they pass sudo test,
                    // the move len will increase. we check that and castle
                    // rights.
                    let mut try_castle_side = |dir, side| {
                        let orig_len = moves.len();
                        try_move!(orig + dir);
                        let rights = *self.get_extra().get_castle(clr, side);
                        // also check that the extra queenside spot is free
                        let q_blocked =
                            side == CastleSide::Long && *self.idx(orig + dir * 3) != Sq(None);
                        if !rights || q_blocked {
                            return;
                        }
                        if moves.len() > orig_len {
                            try_move!(orig + dir * 2);
                        }
                    };
                    try_castle_side(card::W, CastleSide::Long);
                    try_castle_side(card::E, CastleSide::Short);
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
    // find move with matching to_str
    pub fn find_move(&mut self, mv_str: &str) -> Option<MvMeta> {
        let moves = self.get_moves();
        let names: Vec<_> = moves.iter().map(|meta| meta.mv.to_string()).collect();
        moves
            .iter()
            .find(|meta| meta.mv.to_string() == mv_str)
            .copied()
    }
    // make matching moves in sequence
    pub fn run_moves<'a>(&mut self, moves_str: impl Iterator<Item = &'a str>) {
        for mv_str in moves_str {
            match self.find_move(mv_str) {
                Some(meta) => self.make_move(meta.mv),
                None => panic!("no matching move {}", mv_str),
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_move(moves_str: &str) -> Option<Move> {
        let mut state = State::default();
        let mut moves: Vec<&str> = moves_str.split(" ").collect();
        let last = moves.pop().unwrap();

        state.run_moves(moves.iter().map(|&s| s));

        state.find_move(last).map(|meta| meta.mv)
    }
    #[test]
    fn king_into_check() {
        assert!(test_move("b2b3 e7e5 c1a3 e8e7").is_none());
    }
    #[test]
    fn wacky_bongcloud() {
        assert!(test_move("d2d3 a7a5 e1d2").is_some());
    }
}
