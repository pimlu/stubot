use std::ops;

extern crate derive_more;
// use the derives that you want in the file
use derive_more::{Add, AddAssign, From, Into, Neg, Sub, SubAssign};

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

pub struct Move {
    pub a: Pos,
    pub b: Pos,
}
