use super::*;

#[test]
fn my_test() {
    let mut board: chess::State = Default::default();
    let orig = board.clone();
    let moves = board.get_moves();
    assert_eq!(moves.len(), 20);
    assert_eq!(orig, board);
}
