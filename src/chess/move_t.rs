use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos {
    pub y: i8,
    pub x: i8,
}

mod dir {
    use super::Pos;
    const N: Pos = Pos { x: 0, y: 1 };
    const E: Pos = Pos { x: 1, y: 0 };
    const S: Pos = Pos { x: 0, y: -1 };
    const W: Pos = Pos { x: -1, y: 0 };
}

impl ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, _rhs: Pos) -> Pos {
        Pos {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}
impl ops::Neg for Pos {
    type Output = Pos;
    fn neg(self) -> Pos {
        Pos {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl ops::Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, _rhs: Pos) -> Pos {
        self + -_rhs
    }
}

pub struct Move {
    pub a: Pos,
    pub b: Pos,
}
