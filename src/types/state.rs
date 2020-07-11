pub trait StateHistory: PartialEq {
    type State: Copy + Clone;
    type Move;
    fn get_top(&self) -> Self::State;
    fn make_move(&mut self, mv: &Self::Move);
    fn unmake_move(&mut self);
    fn init_state() -> Self;
}

pub trait MoveGen {
    type Move;
    fn get_moves(&self) -> Vec<Self::Move>;
}
