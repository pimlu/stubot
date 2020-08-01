use super::card;
use super::*;

use derive_more::{Add, AddAssign};

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
pub trait MoveAdd: FnMut(Move) {}
impl<T: FnMut(Move)> MoveAdd for T {}

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

#[derive(Debug, Copy, Clone, PartialEq, Default, Add, AddAssign)]
pub struct Perft {
    pub nodes: u128,
    pub caps: u128,
    pub enps: u128,
    pub castles: u128,
    pub promotions: u128,
}

impl State {
    pub fn in_check(&self, clr: Color) -> bool {
        let king_pos = *self.get_king_pos(clr);
        debug_assert!(*self.idx(king_pos) == Sq::new(clr, Type::King));
        self.is_attacked(king_pos, clr.other())
    }
    fn is_attacked(&self, orig: Pos, enemy: Color) -> bool {
        use std::cell::Cell;
        let enemy_knight = Sq::new(enemy, Type::Knight);
        for &opt in KNIGHT_OPTS {
            if self.get(orig + opt) == Some(&enemy_knight) {
                return true;
            }
        }
        // reverse the pawn attack direction, this is relative to the king
        let pdir = pawn_dir(enemy.other());
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
    fn try_move(&self, gate: impl Fn(bool) -> bool, add: &mut impl MoveAdd, mut mv: Move) -> bool {
        if mv.extra == Some(MvExtra::EnPassant) {
            mv.capture = Some(Type::Pawn);
        }
        let b_sq = match self.get(mv.b) {
            Some(sq) => *sq,
            None => return false,
        };
        if !gate(b_sq.0.is_some()) {
            return false;
        }

        let Piece { clr, typ } = match b_sq {
            Sq(Some(pc)) => pc,
            Sq(None) => {
                add(mv);
                return true;
            }
        };
        if clr != self.turn() {
            mv.capture = Some(typ);
            add(mv);
        }
        return false;
    }

    pub fn is_legal(&self) -> bool {
        !self.in_check(self.turn().other())
    }

    pub fn is_legal_move(&mut self, mv: Move) -> bool {
        self.make_move(mv);
        // only extra condition for a psuedo move is check
        let legal = self.is_legal();
        self.unmake_move();
        legal
    }

    // average number of chess moves for Vec::with_capacity
    pub fn move_count(&self) -> usize {
        32
    }

    // requires a mutable reference, but doesn't actually modify anything
    // (if our code is correct)
    pub fn gen_moves(&mut self) -> Vec<Move> {
        let mut moves = self.gen_sudo_moves();
        moves.retain(|&mv| self.is_legal_move(mv));
        moves
    }

    pub fn gen_sudo_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(self.move_count());
        self.add_sudo_moves(&mut |mv| moves.push(mv));
        moves
    }

    pub fn add_sudo_moves(&self, add: &mut impl MoveAdd) {
        let enp = self.get_extra().enp;
        let mut add_moves = |orig| {
            let Piece { clr, typ } = match self.get(orig).unwrap() {
                Sq(Some(pc)) => *pc,
                Sq(None) => return,
            };
            macro_rules! try_move {
                ($gate: expr, $extra: expr) => {
                    |pos| {
                        self.try_move(
                            $gate,
                            add,
                            Move {
                                a: orig,
                                b: pos,
                                capture: None,
                                extra: $extra,
                            },
                        )
                    }
                };
            }
            macro_rules! pawn_move {
                ($pos:expr, $is_take:expr) => {
                    if orig.y == rel_y(clr.other(), 1) {
                        for &typ in &[Type::Knight, Type::Bishop, Type::Rook, Type::Queen] {
                            try_move!(|take| take == $is_take, Some(MvExtra::Promote(typ)))($pos);
                        }
                        false
                    } else {
                        try_move!(|take| take == $is_take, None)($pos)
                    }
                };
            }
            macro_rules! add_move {
                () => {
                    try_move!(|_| true, None)
                };
                ($pos: expr) => {
                    add_move!()($pos)
                };
                ($pos: expr, $extra: expr) => {
                    try_move!(|_| true, $extra)($pos)
                };
            }
            if clr != self.turn() {
                return;
            }

            match typ {
                Type::Pawn => {
                    let dir = pawn_dir(clr);
                    // move-onlies
                    let first_push = pawn_move!(orig + dir, false);
                    if first_push && orig.y == rel_y(clr, 1) {
                        pawn_move!(orig + dir * 2, false);
                    }

                    // diagonal
                    for &side in &[card::E, card::W] {
                        let take_pos = orig + dir + side;
                        // needs to be able to "take" the spot they skipped
                        let y_match = orig.y == rel_y(clr.other(), 3);
                        if enp >= 0 && take_pos.x == enp && y_match {
                            add_move!(take_pos, Some(MvExtra::EnPassant));
                        } else {
                            pawn_move!(take_pos, true);
                        }
                    }
                }
                Type::Knight => leaper(orig, KNIGHT_OPTS, add_move!()),
                Type::Bishop => rider(orig, BISHOP_OPTS, add_move!()),
                Type::Rook => rider(orig, ROOK_OPTS, add_move!()),
                Type::Queen => {
                    rider(orig, BISHOP_OPTS, add_move!());
                    rider(orig, ROOK_OPTS, add_move!());
                }
                Type::King => {
                    leaper(orig, BISHOP_OPTS, add_move!());
                    leaper(orig, ROOK_OPTS, add_move!());
                    let mut try_castle_side = |dir, side| {
                        // must have castle rights
                        if !*self.get_extra().get_castle(clr, side) {
                            return;
                        }
                        // loop across the rook path, should be clear
                        let (src, mut dst) = castle_rook_path(clr, side);
                        while dst != src {
                            if *self.idx(dst) != Sq(None) {
                                return;
                            }
                            dst += dir;
                        }
                        // lastly, can't castle out of/through check
                        if !self.in_check(clr) && !self.is_attacked(orig + dir, clr.other()) {
                            add_move!(orig + dir * 2, Some(MvExtra::Castle(side)));
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
    }
    // find move with matching to_str
    pub fn find_move(&mut self, mv_str: &str) -> Option<Move> {
        let moves = self.gen_moves();
        moves.iter().find(|mv| mv.to_string() == mv_str).copied()
    }
    // make matching moves in sequence
    pub fn run_moves<'a>(&mut self, moves_str: impl Iterator<Item = &'a str>) {
        for mv_str in moves_str.filter(|s| !s.is_empty()) {
            match self.find_move(mv_str) {
                Some(mv) => self.make_move(mv),
                None => panic!("no matching move {}", mv_str),
            };
        }
    }
    pub fn perft(&mut self, depth: u32) -> Perft {
        let mut res: Perft = Default::default();
        if depth == 0 {
            res.nodes = 1;
            return res;
        }

        for mv in self.gen_sudo_moves() {
            #[cfg(test)]
            let mut cpy = self.clone();
            self.make_move(mv);
            if self.is_legal() {
                if depth == 1 {
                    res.nodes += 1;
                    match mv.extra {
                        Some(MvExtra::Castle(_)) => res.castles += 1,
                        Some(MvExtra::EnPassant) => res.enps += 1,
                        Some(MvExtra::Promote(_)) => res.promotions += 1,
                        None => (),
                    }
                    if let Some(_) = mv.capture {
                        res.caps += 1;
                    }
                } else {
                    res += self.perft(depth - 1);
                }
            }
            self.unmake_move();

            // kinda expensive unmake comparison test
            #[cfg(test)]
            if *self != cpy {
                println!("orig:");
                println!("{}", cpy.board_string());
                println!("then move {}:", mv);
                println!("{:?}", mv);
                cpy.make_move(mv);
                println!("{}", cpy.board_string());
                println!("unmade into:");
                println!("{}", self.board_string());
                assert!(false);
            }
        }
        res
    }

    pub fn perftree(&mut self, depth: u32) -> String {
        let mut sum: u128 = 0;
        let mut moves: Vec<_> = self
            .gen_moves()
            .iter()
            .map(|&mv| {
                self.make_move(mv);
                let nodes = self.perft(depth - 1).nodes;
                self.unmake_move();

                sum += nodes;
                (mv.to_string(), nodes)
            })
            .collect();

        moves.sort_by_key(|tup| tup.0.to_string());

        format!(
            "{}\n\n{}",
            moves
                .iter()
                .map(|(mv, n)| format!("{} {}", mv, n))
                .collect::<Vec<_>>()
                .join("\n"),
            sum
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const POS_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const POS_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const POS_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const POS_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
    const DUB_M8: &str = "r3kb2/pp2qp2/2n2B2/8/2B1P3/2N2r2/PPPQ3P/2KR3R b q - 0 16";

    fn test_move(st: Option<&str>, moves_str: &str) -> Option<Move> {
        let mut state: State = st
            .and_then(|s| Some(str::parse(s).unwrap()))
            .unwrap_or_default();
        let mut moves: Vec<&str> = moves_str.split(" ").collect();
        let last = moves.pop().unwrap();

        state.run_moves(moves.iter().map(|&s| s));

        state.find_move(last)
    }
    #[test]
    fn king_into_check() {
        assert!(test_move(None, "b2b3 e7e5 c1a3 e8e7").is_none());
    }
    #[test]
    fn wacky_bongcloud() {
        assert!(test_move(None, "d2d3 a7a5 e1d2").is_some());
    }
    #[test]
    fn kiwipete_pawn_no_castle() {
        assert!(test_move(Some(KIWIPETE), "a1b1 h3g2 e1g1").is_none());
    }
    #[test]
    fn kiwipete_castle_out_of_check() {
        assert!(test_move(Some(KIWIPETE), "a1b1 f6d5 f3f7 e8c8").is_none());
    }
    #[test]
    fn kiwipete_no_castle_take() {
        assert!(test_move(Some(KIWIPETE), "e2a6 b4b3 a6c8 e8c8").is_none());
    }

    fn test_position(state: &mut State, nodes: Vec<u128>) {
        use pretty_assertions::assert_eq;
        for (d, &n) in nodes.iter().enumerate() {
            let count = state.perft((d + 1) as u32).nodes;
            assert_eq!(n, count);
        }
    }
    fn test_precise(state: &mut State, nodes: Vec<Perft>) {
        use pretty_assertions::assert_eq;
        for (d, &n) in nodes.iter().enumerate() {
            let count = state.perft((d + 1) as u32);
            assert_eq!(n, count);
        }
    }

    #[test]
    fn test_initial() {
        let mut state: State = Default::default();
        test_position(&mut state, vec![20, 400, 8902]);
    }

    #[test]
    fn test_kiwipete() {
        let mut state: State = str::parse(KIWIPETE).unwrap();
        test_precise(
            &mut state,
            vec![
                Perft {
                    nodes: 48,
                    caps: 8,
                    enps: 0,
                    castles: 2,
                    promotions: 0,
                },
                Perft {
                    nodes: 2039,
                    caps: 351,
                    enps: 1,
                    castles: 91,
                    promotions: 0,
                },
                Perft {
                    nodes: 97862,
                    caps: 17102,
                    enps: 45,
                    castles: 3162,
                    promotions: 0,
                },
            ],
        );
    }
    #[test]
    fn test_pos_3() {
        let mut state: State = str::parse(POS_3).unwrap();
        test_position(&mut state, vec![14, 191, 2812, 43238]);
    }
    #[test]
    fn test_pos_4() {
        let mut state: State = str::parse(POS_4).unwrap();
        test_position(&mut state, vec![6, 264, 9467]);
    }
    #[test]
    fn test_pos_5() {
        let mut state: State = str::parse(POS_5).unwrap();
        test_position(&mut state, vec![44, 1486, 62379]);
    }
    #[test]
    fn test_pos_6() {
        let mut state: State = str::parse(POS_6).unwrap();
        test_position(&mut state, vec![46, 2079, 89890]);
    }
    // none of the 6 test positions have imminent mate threats in the first 3
    // nodes. add a game that has mate threats so it can be counted
    #[test]
    fn test_dub_m8() {
        let mut state: State = str::parse(DUB_M8).unwrap();
        test_position(&mut state, vec![36, 1715, 62457]);
    }
}
