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
pub struct State {
    ply: i32,
    pub board: [[Sq; BOARD_DIM.x as usize]; BOARD_DIM.y as usize],
    pub castle: [[bool; 2]; 2],
    pub enpassant: i8,
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

impl State {
    // 2d array idx at pos
    pub fn get(&self, i: Pos) -> Option<&Sq> {
        self.board
            .get(i.y as usize)
            .and_then(|r| r.get(i.x as usize))
    }
    pub fn get_mut(&mut self, i: Pos) -> Option<&mut Sq> {
        self.board
            .get_mut(i.y as usize)
            .and_then(|r| r.get_mut(i.x as usize))
    }
    // every other turn, 0 starts at white.
    pub fn turn(&self) -> Color {
        if self.ply % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }
    // in-place make move, returns capture if it ended up taking one.
    // only performs basic sanity checks. this simply writes the result
    // of movegen to the board
    pub fn make_move(&mut self, mut mv: Move) -> Option<Type> {
        let mut a_pc = match self.get(mv.a) {
            Some(Sq(Some(p))) => *p,
            _ => panic!("bad spot a"),
        };

        // ensure we are allowed to move the piece
        debug_assert!(a_pc.clr == self.turn());

        let b_sq = self.get_mut(mv.b).unwrap();

        // ensure we are not bumping into our own piece
        debug_assert!(match b_sq {
            Sq(Some(pc)) => pc.clr != a_pc.clr,
            Sq(None) => true,
        });
        // store capture
        let ret = match b_sq {
            Sq(Some(pc)) => Some(pc.typ),
            Sq(None) => None,
        };

        match mv.extra {
            Some(MvExtra::EnPassant) => {
                debug_assert!(a_pc.typ == Type::Pawn && ret == None);
            }
            Some(MvExtra::Promote(typ)) => a_pc.typ = typ,
            Some(MvExtra::Castle(side)) => (),
            None => (),
        }

        *b_sq = Sq(Some(a_pc));
        *self.get_mut(mv.a).unwrap() = Sq(None);

        return ret;
    }
    pub fn unmake_move(&mut self) {}
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
            ply: 0,
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
            castle: [[true; 2]; 2],
            enpassant: -1,
        };
    }
}
