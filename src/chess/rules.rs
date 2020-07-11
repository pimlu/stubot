use super::*;
use crate::types;

use std::iter;

struct BoardHistory {
    history: Vec<State>,
}

impl types::MoveGen for State {
    type Move = Move;
    fn get_moves(&self) -> Vec<Self::Move> {
        vec![]
    }
}
