use std::fmt;

use super::*;

// row major
pub const BOARD_DIM: Pos = Pos { x: 8, y: 8 };

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Pawn => write!(f, "P"),
            Type::Knight => write!(f, "N"),
            Type::Bishop => write!(f, "B"),
            Type::Rook => write!(f, "R"),
            Type::Queen => write!(f, "Q"),
            Type::King => write!(f, "K"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sq(pub Option<Piece>);
impl Sq {
    fn new(clr: Color, typ: Type) -> Sq {
        return Sq(Some(Piece { clr, typ }));
    }
    const NIL: Sq = Sq(None);
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StateExtra {
    castle: [[bool; 2]; 2],
    pub capture: Option<Type>,
    pub enp: i8,
}

// am I actually going to bitpack this later? probably not
impl StateExtra {
    pub fn get_castle(&self, clr: Color, side: CastleSide) -> &bool {
        &self.castle[clr as usize][side as usize]
    }
    pub fn set_castle(&mut self, clr: Color, side: CastleSide, state: bool) {
        self.castle[clr as usize][side as usize] = state;
    }
}

impl Default for StateExtra {
    fn default() -> Self {
        StateExtra {
            capture: None,
            castle: [[true, true], [true, true]],
            enp: -1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    ply: u32,
    board: [[Sq; BOARD_DIM.x as usize]; BOARD_DIM.y as usize],
    extra: Vec<StateExtra>,
    moves: Vec<Move>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn show_iter<I, J>(show: impl Fn(J) -> String, delim: &str, row: I) -> String
        where
            I: IntoIterator<Item = J>,
        {
            row.into_iter().map(show).collect::<Vec<_>>().join(delim)
        };
        let show_row = |row| show_iter(|e| format!("{}", e), " ", row);
        let s = show_iter(show_row, "\n", self.board.iter().rev());
        write!(f, "{}", s)
    }
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
    pub fn score(&self) -> i16 {
        0
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
}

impl Default for State {
    fn default() -> Self {
        const W: usize = BOARD_DIM.x as usize;

        let make_set = |c, ts: [Type; W]| {
            let mut squares = [Sq::NIL; W];
            for (i, t) in ts.iter().enumerate() {
                squares[i] = Sq::new(c, *t);
            }
            squares
        };
        let make_backline = |c| {
            make_set(
                c,
                [
                    Type::Rook,
                    Type::Knight,
                    Type::Bishop,
                    Type::Queen,
                    Type::King,
                    Type::Bishop,
                    Type::Knight,
                    Type::Rook,
                ],
            )
        };
        let make_pawns = |c| [Sq::new(c, Type::Pawn); W];
        let empty = [Sq::NIL; W];

        return State {
            ply: 1,
            board: [
                make_backline(Color::White),
                make_pawns(Color::White),
                empty,
                empty,
                empty,
                empty,
                make_pawns(Color::Black),
                make_backline(Color::Black),
            ],
            extra: vec![Default::default()],
            moves: vec![],
        };
    }
}
