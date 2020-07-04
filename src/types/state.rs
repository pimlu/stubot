trait StateHistory: PartialEq {
    type State: Copy + Clone;
    type Move;
    fn get_top() -> Self::State;
    fn make_move(mv: &Self::Move);
    fn unmake_move();
}
