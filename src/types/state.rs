pub trait StateHistory: PartialEq {
    type Move;
    fn make_move(&mut self, mv: &Self::Move);
    fn unmake_move(&mut self);
}

pub trait MoveGen {
    type Move;
    fn get_moves(&self) -> Vec<Self::Move>;
}
