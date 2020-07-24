extern crate derive_more;

use derive_more::{Add, AddAssign, From, Into, Neg, Sub, SubAssign};
use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
pub enum Color {
    White = 0,
    Black,
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

#[derive(Debug, Copy, Clone, PartialEq, Add, AddAssign, Sub, SubAssign, Neg, From, Into)]
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
    pub extra: Option<MvExtra>,
}
