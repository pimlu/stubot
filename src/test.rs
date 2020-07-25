use super::*;

#[test]
fn my_test() {
    let mut state: chess::State = Default::default();
    let orig = state.clone();
    let moves = state.get_moves();
    assert_eq!(moves.len(), 20);
    assert_eq!(orig, state);
}
