use std::fmt;

use super::*;
use crate::types;

// row major
pub const BOARD_DIM: Pos = Pos { x: 8, y: 8 };

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

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
pub struct Piece {
    pub c: Color,
    pub t: Type,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sq(pub Option<Piece>);
impl Sq {
    fn new(c: Color, t: Type) -> Sq {
        return Sq(Some(Piece { c: c, t: t }));
    }
    const NIL: Sq = Sq(None);
}

impl fmt::Display for Sq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(p) => {
                let s = p.t.to_string();
                write!(
                    f,
                    "{}",
                    if p.c == Color::White {
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
    pub turn: Color,
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
    pub fn get(&self, i: Pos) -> Option<&Sq> {
        self.board
            .get(i.y as usize)
            .and_then(|r| r.get(i.x as usize))
    }
    pub fn init_board() -> State {
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
            turn: Color::White,
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
