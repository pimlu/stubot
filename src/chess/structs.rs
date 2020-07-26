extern crate derive_more;

use derive_more::{Add, AddAssign, From, Into, Mul, Neg, Sub, SubAssign};
use num_derive::FromPrimitive;

use std::fmt;
use std::str;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
pub enum Color {
    White = 0,
    Black,
}
impl Color {
    pub fn other(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
pub enum Type {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
pub enum CastleSide {
    Long = 0,
    Short,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub clr: Color,
    pub typ: Type,
}

#[derive(Debug, Copy, Clone, PartialEq, Add, AddAssign, Sub, SubAssign, Neg, Mul, From, Into)]
pub struct Pos {
    pub y: i8,
    pub x: i8,
}

pub mod card {
    use super::Pos;
    pub const N: Pos = Pos { x: 0, y: 1 };
    pub const E: Pos = Pos { x: 1, y: 0 };
    pub const S: Pos = Pos { x: 0, y: -1 };
    pub const W: Pos = Pos { x: -1, y: 0 };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MvExtra {
    EnPassant,
    Castle(CastleSide),
    Promote(Type),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub a: Pos,
    pub b: Pos,
    pub capture: Option<Type>,
    pub extra: Option<MvExtra>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sq(pub Option<Piece>);

impl Sq {
    pub fn new(clr: Color, typ: Type) -> Self {
        Sq(Some(Piece { clr, typ }))
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub name: String,
}

impl ParseError {
    pub fn new(name: &str) -> Self {
        ParseError {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse {} error", self.name)
    }
}

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

// super lazy way to parse
impl str::FromStr for Sq {
    type Err = ParseError;

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
        Err(ParseError::new("Sq"))
    }
}
