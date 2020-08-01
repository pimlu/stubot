use super::*;

type ScoreTable = [[i8; BOARD_DIM.x as usize]; BOARD_DIM.y as usize];

// thank you cargo fmt for making these readable
const PAWN_TBL: ScoreTable = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, -20, -20, 10, 10, 5],
    [5, -5, -10, 0, 0, -10, -5, 5],
    [0, 0, 0, 20, 20, 0, 0, 0],
    [5, 5, 10, 25, 25, 10, 5, 5],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

const KNIGHT_TBL: ScoreTable = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-30, 0, 10, 15, 15, 10, 0, -30],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 10, 15, 15, 10, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];

const BISHOP_TBL: ScoreTable = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];

const ROOK_TBL: ScoreTable = [
    [0, 0, 0, 5, 5, 0, 0, 0],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

const QUEEN_TBL: ScoreTable = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];

const KING_TBL: ScoreTable = [
    [20, 30, 10, 0, 0, 10, 30, 20],
    [20, 20, 0, 0, 0, 0, 20, 20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
];

pub const CHECKMATE: i16 = 20000;
pub const DRAW: i16 = 0;
const TYP_VALS: &[i16] = &[100, 320, 330, 500, 900, 0];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FastEval {
    piece_counts: [[u8; 5]; 2],
    score: i16,
    king_early: i16,
    king_late: i16,
}
impl Default for FastEval {
    fn default() -> Self {
        FastEval {
            piece_counts: [[0; 5]; 2],
            score: 0,
            king_early: 0,
            king_late: 0,
        }
    }
}
impl FastEval {
    pub fn change(&mut self, add: bool, sq: Sq, pos: Pos) {
        let pc = match sq {
            Sq(Some(pc)) => pc,
            Sq(None) => return,
        };
        let typ_val = TYP_VALS[pc.typ as usize];

        let pos_val = match pc.typ {
            Type::Pawn => PAWN_TBL,
            Type::Knight => KNIGHT_TBL,
            Type::Bishop => BISHOP_TBL,
            Type::Rook => ROOK_TBL,
            Type::Queen => QUEEN_TBL,
            Type::King => KING_TBL,
        }[rel_y(pc.clr, pos.y) as usize][pos.x as usize];

        let score = typ_val + pos_val as i16;

        let mut diff = match pc.clr {
            Color::White => score,
            Color::Black => -score,
        };
        if !add {
            diff = -diff;
        }
        if pc.typ == Type::King {
            self.king_early += diff;
        } else {
            self.score += diff;
        }
    }
    pub fn score(&self) -> i16 {
        return self.score + self.king_early;
    }
}
