use super::*;

use core::fmt;
use core::str;

use alloc::string::*;
use alloc::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StateExtra {
    castle: [[bool; 2]; 2],
    pub capture: Option<Type>,
    pub enp: Option<Pos>,
}

impl StateExtra {
    fn zero_init() -> Self {
        StateExtra {
            castle: [[false; 2]; 2],
            capture: None,
            enp: None,
        }
    }
    pub fn get_castle(&self, clr: Color, side: CastleSide) -> &bool {
        &self.castle[clr as usize][side as usize]
    }
    pub fn set_castle(&mut self, clr: Color, side: CastleSide, state: bool) {
        self.castle[clr as usize][side as usize] = state;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    ply: u32,
    ply_clock: u32,
    board: [[Sq; BOARD_DIM.x as usize]; BOARD_DIM.y as usize],
    king_pos: [Pos; 2],
    cur_extra: StateExtra,
    extras: Vec<StateExtra>,
    moves: Vec<Move>,
    fast_eval: FastEval,
}
// returns x value (src, dst)
pub fn castle_rook_path(clr: Color, side: CastleSide) -> (Pos, Pos) {
    let (src, dst) = match side {
        CastleSide::Long => (0, 3),
        CastleSide::Short => (7, 5),
    };
    let y = rel_y(clr, 0);
    (Pos { x: src, y }, Pos { x: dst, y })
}
// gets the position of the taken pawn from en passant
fn en_passant_cap(mv: Move) -> Pos {
    // x/col of dest sq, y/row of src sq
    Pos {
        x: mv.b.x,
        y: mv.a.y,
    }
}

impl State {
    // 2d array idx at pos
    pub fn get(&self, i: Pos) -> Option<&Sq> {
        self.board
            .get(i.y as usize)
            .and_then(|r| r.get(i.x as usize))
    }
    fn get_mut(&mut self, i: Pos) -> Option<&mut Sq> {
        self.board
            .get_mut(i.y as usize)
            .and_then(|r| r.get_mut(i.x as usize))
    }
    pub fn idx(&self, pos: Pos) -> &Sq {
        self.get(pos).unwrap()
    }
    fn set(&mut self, pos: Pos, x: Sq) {
        self.fast_eval.change(false, *self.idx(pos), pos);
        self.fast_eval.change(true, x, pos);
        let sq = self.get_mut(pos).unwrap();
        *sq = x;
        let Piece { clr, typ } = match x {
            Sq(Some(pc)) => pc,
            Sq(None) => return,
        };
        if typ == Type::King {
            self.set_king_pos(clr, pos);
        }
    }

    // every other turn, 0 starts at white.
    pub fn turn(&self) -> Color {
        if self.ply % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }
    fn commit_extra(&mut self, extra: StateExtra) {
        self.cur_extra = extra;
    }
    pub fn get_extra(&self) -> &StateExtra {
        &self.cur_extra
    }
    pub fn get_king_pos(&self, clr: Color) -> &Pos {
        &self.king_pos[clr as usize]
    }
    pub fn set_king_pos(&mut self, clr: Color, pos: Pos) {
        self.king_pos[clr as usize] = pos;
    }
    pub fn move_len(&self) -> usize {
        self.moves.len()
    }

    pub fn rel_neg(&self, score: i16) -> i16 {
        self.turn().rel_neg(score)
    }
    pub fn fast_score(&self) -> i16 {
        self.fast_eval.score()
    }
    pub fn slow_score(&mut self) -> i16 {
        let moves = self.gen_sudo_moves();
        for mv in moves {
            if self.is_legal_move(mv) {
                return self.fast_score();
            }
        }
        // we have no legal moves.
        self.end_score()
    }
    pub fn end_score(&self) -> i16 {
        if self.in_check(self.turn()) {
            self.rel_neg(-CHECKMATE)
        } else {
            DRAW
        }
    }
    // in-place make move.
    // only performs basic sanity checks. this simply writes the result
    // of movegen to the board
    pub fn make_move(&mut self, mut mv: Move) {
        // extra moves/metadata is per make_move, should match
        debug_assert!(self.extras.len() == self.moves.len());
        // copy extra data and push
        self.extras.push(self.cur_extra);
        self.moves.push(mv);

        // moving from a to b
        let mut a_pc = self.idx(mv.a).0.unwrap();
        let &b_sq = self.idx(mv.b);

        // ensure we are allowed to move the piece
        debug_assert!(a_pc.clr == self.turn());

        // ensure we are not bumping into our own piece
        debug_assert!(match b_sq {
            Sq(Some(pc)) => pc.clr != a_pc.clr,
            Sq(None) => true,
        });

        match mv.extra {
            Some(MvExtra::EnPassant) => {
                debug_assert!(a_pc.typ == Type::Pawn && b_sq == Sq(None));
                mv.capture = None;
                self.set(en_passant_cap(mv), Sq(None));
            }
            Some(MvExtra::Promote(typ)) => a_pc.typ = typ,
            Some(MvExtra::Castle(side)) => {
                let (src, dst) = castle_rook_path(self.turn(), side);
                self.set(src, Sq(None));
                self.set(dst, Sq::new(self.turn(), Type::Rook));
            }
            None => (),
        }
        let mut st_extra = self.cur_extra;
        st_extra.enp = None;
        match a_pc.typ {
            Type::Pawn =>
            // prep for en passant next move
            {
                if (mv.b.y - mv.a.y).abs() == 2 {
                    st_extra.enp = Some(mv.b);
                }
            }
            Type::King => {
                st_extra.set_castle(self.turn(), CastleSide::Long, false);
                st_extra.set_castle(self.turn(), CastleSide::Short, false);
            }
            Type::Rook => {
                for &side in &[CastleSide::Long, CastleSide::Short] {
                    // if the rook is moving away from its original position
                    if mv.a == castle_rook_path(self.turn(), side).0 {
                        st_extra.set_castle(self.turn(), side, false)
                    }
                }
            }
            _ => (),
        }
        // if you take someone's rook, reset castling rights
        if mv.capture == Some(Type::Rook) {
            let enemy = self.turn().other();
            for &side in &[CastleSide::Long, CastleSide::Short] {
                if mv.b == castle_rook_path(enemy, side).0 {
                    st_extra.set_castle(enemy, side, false);
                }
            }
        }

        self.commit_extra(st_extra);

        // move the pieces
        self.set(mv.a, Sq(None));
        self.set(mv.b, Sq(Some(a_pc)));

        // don't change self.turn() till the end
        self.ply += 1;
    }
    pub fn unmake_move(&mut self) {
        self.ply -= 1;
        let st_extra = self.extras.pop().unwrap();

        let mut mv = self.moves.pop().unwrap();

        // moving from b to a
        let mut b_pc = self.idx(mv.b).0.unwrap();

        // we came from a, it should be empty
        debug_assert!(*self.idx(mv.a) == Sq(None));

        let enemy_turn = self.turn().other();
        let enemy_sq = |typ| Sq::new(enemy_turn, typ);

        match mv.extra {
            Some(MvExtra::EnPassant) => {
                // don't restore a pawn at the wrong spot
                mv.capture = None;
                // restore it at enp spot instead
                self.set(en_passant_cap(mv), enemy_sq(Type::Pawn));
            }
            Some(MvExtra::Promote(_)) => b_pc.typ = Type::Pawn,
            Some(MvExtra::Castle(side)) => {
                let (src, dst) = castle_rook_path(self.turn(), side);
                self.set(dst, Sq(None));
                self.set(src, Sq::new(self.turn(), Type::Rook));
            }
            None => (),
        }

        self.commit_extra(st_extra);

        // move the pieces, restoring a capture
        self.set(mv.a, Sq(Some(b_pc)));
        self.set(
            mv.b,
            match mv.capture {
                Some(typ) => enemy_sq(typ),
                None => Sq(None),
            },
        );
    }
    pub fn zero_board() -> Self {
        State {
            ply: 0,
            ply_clock: 0,
            board: [[Sq(None); BOARD_DIM.x as usize]; BOARD_DIM.y as usize],
            king_pos: [Pos { x: 0, y: 0 }, Pos { x: 0, y: 0 }],
            cur_extra: StateExtra::zero_init(),
            extras: vec![],
            moves: vec![],
            fast_eval: Default::default(),
        }
    }
}

// String processing stuff

// maps an iterator and joins it on delim
pub fn show_iter<I, J>(show: impl Fn(J) -> String, delim: &str, row: I) -> String
where
    I: IntoIterator<Item = J>,
{
    row.into_iter().map(show).collect::<Vec<_>>().join(delim)
}
impl State {
    pub fn board_string(&self) -> String {
        let show_row = |row| show_iter(|e| format!("{}", e), " ", row);
        show_iter(show_row, "\n", self.board.iter().rev())
    }
}
// output fen format
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pos = show_iter(
            |row| {
                let mut count: u8 = 0;
                let mut s = String::new();
                let dump = |count: &mut u8| {
                    let d = ('0' as u8 + *count) as char;
                    *count = 0;
                    if d == '0' {
                        String::new()
                    } else {
                        d.to_string()
                    }
                };
                for sq in row {
                    s += &match sq.0 {
                        Some(_) => dump(&mut count) + &sq.to_string(),
                        None => {
                            count += 1;
                            String::new()
                        }
                    };
                }
                s + &dump(&mut count)
            },
            "/",
            self.board.iter().rev(),
        );
        let move_num = 1 + self.ply / 2;
        let mut castle_rights = String::with_capacity(4);
        for clr in [Color::White, Color::Black] {
            for (typ, side) in [
                (Type::King, CastleSide::Short),
                (Type::Queen, CastleSide::Long),
            ] {
                if *self.get_extra().get_castle(clr, side) {
                    let pc = format!("{}", Sq::new(clr, typ));
                    castle_rights += &pc;
                }
            }
        }
        if castle_rights.is_empty() {
            castle_rights = "-".to_string();
        }
        let enp = self
            .get_extra()
            .enp
            .map_or("-".to_string(), |e| e.to_string());

        write!(
            f,
            "{} {} {} {} {} {}",
            pos,
            self.turn(),
            castle_rights,
            enp,
            self.ply_clock,
            move_num
        )
    }
}

impl str::FromStr for State {
    type Err = ChessParseError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        // seems like errors are a pain in rust, make the pain go away :)
        fn conv_err<T, E>(res: Result<T, E>) -> Result<T, ChessParseError> {
            match res.ok() {
                Some(x) => Ok(x),
                None => Err(ChessParseError::new("FEN")),
            }
        }
        let p_sq = |c: char| conv_err(str::parse::<Sq>(&c.to_string()));

        let mut state = State::zero_board();
        let items = fen.split(" ").collect::<Vec<_>>();
        if let [board, turn, castle, enp, half, full] = items.as_slice() {
            let clr_add = match *turn {
                "w" => 0,
                "b" => 1,
                _ => return Err(ChessParseError::new("FEN")),
            };
            let full_u = conv_err(str::parse::<u32>(full))?;
            // full turns are double, we start at ply 0, not full turn 1
            state.ply = 2 * (full_u - 1) + clr_add;
            state.ply_clock = conv_err(str::parse::<u32>(half))?;

            let mut extra = StateExtra::zero_init();
            extra.enp = str::parse::<Pos>(enp).ok();
            for c in castle.chars() {
                let (clr, side) = match c {
                    'q' => (Color::Black, CastleSide::Long),
                    'k' => (Color::Black, CastleSide::Short),
                    'Q' => (Color::White, CastleSide::Long),
                    'K' => (Color::White, CastleSide::Short),
                    '-' => continue,
                    _ => return Err(ChessParseError::new("FEN")),
                };
                extra.set_castle(clr, side, true);
            }
            state.commit_extra(extra);

            for (y, row) in board.rsplit("/").enumerate() {
                if y >= BOARD_DIM.y as usize {
                    return Err(ChessParseError::new("FEN"));
                }
                let mut x: usize = 0;
                for c in row.chars() {
                    if "123456789".contains(c) {
                        x += c as usize - '0' as usize;
                    } else {
                        if x >= BOARD_DIM.x as usize {
                            return Err(ChessParseError::new("FEN"));
                        }
                        state.set(
                            Pos {
                                x: x as i8,
                                y: y as i8,
                            },
                            p_sq(c)?,
                        );
                        x += 1;
                    }
                }
            }

            Ok(state)
        } else {
            Err(ChessParseError::new("FEN"))
        }
    }
}

impl Default for State {
    fn default() -> Self {
        str::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::{testpos::*, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn correct_init() {
        assert_eq!(
            State::default().board_string(),
            "r n b q k b n r
p p p p p p p p
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
P P P P P P P P
R N B Q K B N R"
        );
    }
    #[test]
    fn fen_parse_matches() {
        for fen in [KIWIPETE, POS_3, POS_4, POS_5, POS_6, DUB_M8] {
            assert_eq!(fen, str::parse::<State>(fen).unwrap().to_string());
        }
    }
}
