use super::*;

pub fn perft(state: &mut chess::State, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = state.get_moves();
    moves
        .iter()
        .map(|meta| {
            state.make_move(meta.mv);
            let nodes = perft(state, depth - 1);
            state.unmake_move();
            nodes
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_initial() {
        let mut state: chess::State = Default::default();
        test_position(&mut state, vec![20, 400, 8902]);
    }

    fn test_position(state: &mut chess::State, nodes: Vec<u64>) {
        let orig = state.clone();
        for (d, &n) in nodes.iter().enumerate() {
            let count = cmds::perft(state, (d + 1) as u32);
            assert_eq!(orig, *state);
            assert_eq!(n, count);
        }
    }
}
