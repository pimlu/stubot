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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MvMeta {
    pub mv: Move,
    pub score: i16,
}

impl State {
    fn add_sudo(&mut self, moves: &mut Vec<MvMeta>, mv: Move) {
        #[cfg(test)]
        let mut cpy = self.clone();

        self.make_move(mv);
        moves.push(MvMeta { mv, score: 0 });
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
    fn in_check(&self, clr: Color) -> bool {
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
    fn try_move(
        &mut self,
        gate: impl Fn(bool) -> bool,
        moves: &mut Vec<MvMeta>,
        mut mv: Move,
    ) -> bool {
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
    pub fn is_legal(&mut self, mv: Move) -> bool {
        self.make_move(mv);
        // only extra condition for a psuedo move is check
        let legal = !self.in_check(self.turn().other());
        self.unmake_move();
        legal
    }

    pub fn get_moves(&mut self) -> Vec<MvMeta> {
        let mut moves = Vec::new();
        self.get_sudo_moves(&mut moves);
        moves.retain(|meta| self.is_legal(meta.mv));
        moves
    }

    // requires a mutable reference, but doesn't actually modify anything
    // (if our code is correct)
    pub fn get_sudo_moves(&mut self, moves: &mut Vec<MvMeta>) {
        debug_assert!(moves.is_empty());

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
                            moves,
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
    pub fn find_move(&mut self, mv_str: &str) -> Option<MvMeta> {
        let moves = self.get_moves();
        moves
            .iter()
            .find(|meta| meta.mv.to_string() == mv_str)
            .copied()
    }
    // make matching moves in sequence
    pub fn run_moves<'a>(&mut self, moves_str: impl Iterator<Item = &'a str>) {
        for mv_str in moves_str.filter(|s| !s.is_empty()) {
            match self.find_move(mv_str) {
                Some(meta) => self.make_move(meta.mv),
                None => panic!("no matching move {}", mv_str),
            };
        }
    }

    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = self.get_moves();
        moves
            .iter()
            .map(|meta| {
                self.make_move(meta.mv);
                let nodes = self.perft(depth - 1);
                self.unmake_move();
                nodes
            })
            .sum()
    }

    pub fn perftree(&mut self, depth: u32) -> String {
        let mut sum: u64 = 0;
        let mut moves: Vec<_> = self
            .get_moves()
            .iter()
            .map(|meta| {
                let mv = meta.mv;

                self.make_move(mv);
                let nodes = self.perft(depth - 1);
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

    fn test_move(st: Option<&str>, moves_str: &str) -> Option<Move> {
        let mut state: State = st
            .and_then(|s| Some(str::parse(s).unwrap()))
            .unwrap_or_default();
        let mut moves: Vec<&str> = moves_str.split(" ").collect();
        let last = moves.pop().unwrap();

        state.run_moves(moves.iter().map(|&s| s));

        state.find_move(last).map(|meta| meta.mv)
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

    fn test_position(state: &mut State, nodes: Vec<u64>) {
        use pretty_assertions::assert_eq;
        let orig = state.clone();
        for (d, &n) in nodes.iter().enumerate() {
            let count = state.perft((d + 1) as u32);
            assert_eq!(orig, *state);
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
        test_position(&mut state, vec![48, 2039, 97862]);
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
}
