pub trait SearchState {
    type SudoMove;
    type Score;
    fn gen_sudo_moves(&self, moves: &mut Vec<Self::SudoMove>);
    fn try_make_move(mv: Self::SudoMove) -> bool;
    fn fast_score(&self) -> Self::Score;
    fn unmake_move(mv: Self::SudoMove);
    fn is_quiet(&self, mv: Self::SudoMove) -> bool;
}
