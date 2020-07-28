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

pub fn perftree(state: &mut chess::State, depth: u32) -> String {
    let mut sum: u64 = 0;
    let mut moves: Vec<_> = state
        .get_moves()
        .iter()
        .map(|meta| {
            let mv = meta.mv;

            state.make_move(mv);
            let nodes = cmds::perft(state, depth - 1);
            state.unmake_move();

            sum += nodes;
            (mv.to_string(), nodes)
        })
        .collect();

    moves.sort_by_key(|tup| tup.0.to_string());

    format!(
        "{}\n\n{}",
        moves
            .iter()
            .map(|(mv, n)| format!("{} {}", mv, n))
            .collect::<Vec<_>>()
            .join("\n"),
        sum
    )
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

    #[test]
    fn test_kiwipete() {
        let pos = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let mut state: chess::State = str::parse(pos).unwrap();
        test_position(&mut state, vec![48, 2039, 97862]);
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
