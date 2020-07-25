use super::*;

use std::fmt;
use std::str;

// row major
pub const BOARD_DIM: Pos = Pos { x: 8, y: 8 };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sq(pub Option<Piece>);
impl Sq {
    fn new(clr: Color, typ: Type) -> Sq {
        return Sq(Some(Piece { clr, typ }));
    }
    const NIL: Sq = Sq(None);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StateExtra {
    castle: [[bool; 2]; 2],
    pub capture: Option<Type>,
    pub enp: i8,
}

impl StateExtra {
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
    board: [[Sq; BOARD_DIM.x as usize]; BOARD_DIM.y as usize],
    king_pos: [Pos; 2],
    extra: Vec<StateExtra>,
    moves: Vec<Move>,
}

// gets the position of the taken pawn from en passant
fn en_passant_cap(mv: Move) -> Pos {
    // x/col of dest sq, y/row of src sq
    Pos {
        x: mv.b.x,
        y: mv.a.x,
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
    pub fn idx(&self, i: Pos) -> &Sq {
        self.get(i).unwrap()
    }
    fn set(&mut self, i: Pos, x: Sq) {
        let sq = self.get_mut(i).unwrap();
        *sq = x;
    }

    // every other turn, 0 starts at white.
    pub fn turn(&self) -> Color {
        if self.ply % 2 == 0 {
            Color::Black
        } else {
            Color::White
        }
    }
    // in-place make move, returns capture if it ended up taking one.
    // only performs basic sanity checks. this simply writes the result
    // of movegen to the board
    pub fn make_move(&mut self, mv: Move) -> Option<Type> {
        // extra moves/metadata is per-ply, should match
        debug_assert!(self.ply as usize == self.extra.len());
        debug_assert!(self.ply as usize == self.moves.len() + 1);
        // copy extra data and push
        self.extra.push(*self.extra.last().unwrap());
        self.moves.push(mv);

        // moving from a to b
        let mut a_pc = self.idx(mv.a).0.unwrap();
        let b_sq = self.idx(mv.b);

        // ensure we are allowed to move the piece
        debug_assert!(a_pc.clr == self.turn());

        // ensure we are not bumping into our own piece
        debug_assert!(match b_sq {
            Sq(Some(pc)) => pc.clr != a_pc.clr,
            Sq(None) => true,
        });

        // return capture
        let mut cap = match b_sq {
            Sq(Some(pc)) => Some(pc.typ),
            Sq(None) => None,
        };

        match mv.extra {
            Some(MvExtra::EnPassant) => {
                debug_assert!(a_pc.typ == Type::Pawn && cap == None);
                cap = Some(Type::Pawn);
                self.set(en_passant_cap(mv), Sq(None));
            }
            Some(MvExtra::Promote(typ)) => a_pc.typ = typ,
            Some(MvExtra::Castle(_side)) => (),
            None => (),
        }

        // prep for en passant next move
        let st_extra = self.extra.last_mut().unwrap();
        if a_pc.typ == Type::Pawn && (mv.b.y - mv.a.y).abs() == 2 {
            st_extra.enp = mv.a.x;
        } else {
            st_extra.enp = -1;
        }

        // move the pieces
        self.set(mv.a, Sq(None));
        self.set(mv.b, Sq(Some(a_pc)));

        // don't change self.turn() till the end
        self.ply += 1;

        cap
    }
    pub fn unmake_move(&mut self) {
        self.ply -= 1;
        let mut st_extra = self.extra.pop().unwrap();

        let mut mv = self.moves.pop().unwrap();

        // moving from b to a
        let mut b_pc = self.idx(mv.b).0.unwrap();

        // we came from a, it should be empty
        debug_assert!(*self.idx(mv.a) == Sq::NIL);

        let enemy_turn = self.turn().other();
        let enemy_sq = |typ| {
            Sq(Some(Piece {
                clr: enemy_turn,
                typ,
            }))
        };

        match mv.extra {
            Some(MvExtra::EnPassant) => {
                // don't restore a pawn at the wrong spot
                mv.capture = None;
                // restore it at enp spot instead
                self.set(en_passant_cap(mv), enemy_sq(Type::Pawn));
            }
            Some(MvExtra::Promote(_)) => b_pc.typ = Type::Pawn,
            Some(MvExtra::Castle(_side)) => (),
            None => (),
        }

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
            ply: 1,
            board: [[Sq::NIL; BOARD_DIM.x as usize]; BOARD_DIM.y as usize],
            king_pos: [Pos { x: 0, y: 0 }, Pos { x: 0, y: 0 }],
            extra: vec![StateExtra {
                capture: None,
                castle: [[false; 2]; 2],
                enp: -1,
            }],
            moves: vec![],
        }
    }
}

// String processing stuff

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Type::Pawn => "P",
                Type::Knight => "N",
                Type::Bishop => "B",
                Type::Rook => "R",
                Type::Queen => "Q",
                Type::King => "K",
            }
        )
    }
}

impl fmt::Display for Sq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(p) => {
                let s = p.typ.to_string();
                write!(
                    f,
                    "{}",
                    if p.clr == Color::White {
                        s
                    } else {
                        s.to_lowercase()
                    }
                )
            }
            None => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseSqError;

impl fmt::Display for ParseSqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse square error")
    }
}

// super lazy way to parse
impl str::FromStr for Sq {
    type Err = ParseSqError;

    fn from_str(sq_str: &str) -> Result<Self, Self::Err> {
        if sq_str == "." {
            return Ok(Sq(None));
        }
        for &clr in &[Color::White, Color::Black] {
            for &typ in &[
                Type::Pawn,
                Type::Knight,
                Type::Bishop,
                Type::Rook,
                Type::Queen,
                Type::King,
            ] {
                let sq = Sq(Some(Piece { clr, typ }));
                if sq_str == sq.to_string() {
                    return Ok(sq);
                }
            }
        }
        Err(ParseSqError)
    }
}

fn show_iter<I, J>(show: impl Fn(J) -> String, delim: &str, row: I) -> String
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
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "fen")
    }
}

#[derive(Debug, Clone)]
pub struct ParseFenError;

impl fmt::Display for ParseFenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse FEN error")
    }
}

impl str::FromStr for State {
    type Err = ParseFenError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let mut state = State::zero_board();
        let items = fen.split(" ").collect::<Vec<_>>();
        if let [board, turn, castle, enp, _half, full] = items.as_slice() {
            let clr_add = match *turn {
                "w" => 0,
                "b" => 1,
                _ => return Err(ParseFenError),
            };
            // full turns are double, we start at ply 1, not full turn 1
            state.ply = match str::parse::<u32>(full).ok() {
                Some(x) => Ok(2 * x - 1 + clr_add),
                None => Err(ParseFenError),
            }?;

            for (inv_y, row) in board.split("/").enumerate() {
                if inv_y >= BOARD_DIM.y as usize {
                    return Err(ParseFenError);
                }
                let mut x: usize = 0;
                for c in row.chars() {
                    if "12345678".contains(c) {
                        x += c as usize - '0' as usize;
                    } else {
                        if x >= BOARD_DIM.x as usize {
                            return Err(ParseFenError);
                        }
                        let y = BOARD_DIM.y - 1 - inv_y as i8;
                        let sq = match str::parse::<Sq>(&c.to_string()).ok() {
                            Some(x) => Ok(x),
                            None => Err(ParseFenError),
                        }?;
                        state.set(Pos { x: x as i8, y }, sq);
                        x += 1;
                    }
                }
            }

            Ok(state)
        } else {
            Err(ParseFenError)
        }
    }
}

impl Default for State {
    fn default() -> Self {
        str::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}
