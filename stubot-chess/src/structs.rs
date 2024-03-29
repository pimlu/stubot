extern crate derive_more;

use derive_more::{Add, AddAssign, From, Into, Mul, Neg, Sub, SubAssign};

use core::fmt;
use core::str;

use alloc::string::*;

// row major
pub const BOARD_DIM: Pos = Pos { x: 8, y: 8 };

pub fn rel_y(clr: Color, y: i8) -> i8 {
    match clr {
        Color::White => y,
        Color::Black => BOARD_DIM.y - 1 - y,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White = 0,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Color::White => "w",
                Color::Black => "b",
            }
        )
    }
}

impl Color {
    pub fn other(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn rel_neg(&self, score: i16) -> i16 {
        match self {
            Color::White => score,
            Color::Black => -score,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug, Copy, Clone, PartialEq)]
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

impl Pos {
    pub fn color(&self) -> Color {
        if (self.x + self.y) % 2 == 0 {
            Color::Black
        } else {
            Color::White
        }
    }
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
pub struct ChessParseError {
    pub name: String,
}

impl ChessParseError {
    pub fn new(name: &str) -> Self {
        ChessParseError {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for ChessParseError {
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
                Type::Pawn => "p",
                Type::Knight => "n",
                Type::Bishop => "b",
                Type::Rook => "r",
                Type::Queen => "q",
                Type::King => "k",
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
                        s.to_uppercase()
                    } else {
                        s
                    }
                )
            }
            None => write!(f, "."),
        }
    }
}

// super lazy way to parse
impl str::FromStr for Sq {
    type Err = ChessParseError;

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
        Err(ChessParseError::new("Sq"))
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char_add = |c, p| (c as u8 + p as u8) as char;
        write!(f, "{}{}", char_add('a', self.x), char_add('1', self.y))
    }
}

impl str::FromStr for Pos {
    type Err = ChessParseError;

    fn from_str(pos: &str) -> Result<Self, Self::Err> {
        if pos.len() < 2 {
            return Err(ChessParseError::new("Pos"));
        }

        let x = pos.chars().nth(0).unwrap() as i8 - 'a' as i8;
        let y: i8 = match str::parse(&pos[1..]).ok() {
            Some(v) => Ok(v),
            None => Err(ChessParseError::new("Pos")),
        }?;
        Ok(Pos { x, y: y - 1 })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let promote = match self.extra {
            Some(MvExtra::Promote(typ)) => typ.to_string(),
            _ => "".to_string(),
        };
        write!(f, "{}{}{}", self.a, self.b, promote)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pos_serial() {
        let test_pos = |p: &str| {
            let s = p.to_string();
            assert_eq!(s, str::parse::<Pos>(&s).unwrap().to_string());
        };
        test_pos("a3");
        test_pos("c8");
        test_pos("g1");
        assert_eq!("a1", Pos { x: 0, y: 0 }.to_string());
        assert_eq!("c7", Pos { x: 2, y: 6 }.to_string());
    }

    #[test]
    fn pos_color() {
        let clr = |s| str::parse::<Pos>(s).unwrap().color();
        assert_eq!(clr("a1"), Color::Black);
        assert_eq!(clr("f5"), Color::White);
    }
}
