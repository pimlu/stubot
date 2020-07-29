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
    use crate::chess::consts;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_initial() {
        let mut state: chess::State = Default::default();
        test_position(&mut state, vec![20, 400, 8902]);
    }

    #[test]
    fn test_kiwipete() {
        let mut state: chess::State = str::parse(consts::KIWIPETE).unwrap();
        test_position(&mut state, vec![48, 2039, 97862]);
    }

    #[test]
    fn test_pos_3() {
        let mut state: chess::State = str::parse(consts::POS_3).unwrap();
        test_position(&mut state, vec![14, 191, 2812, 43238]);
    }
    #[test]
    fn test_pos_4() {
        let mut state: chess::State = str::parse(consts::POS_4).unwrap();
        test_position(&mut state, vec![6, 264, 9467]);
    }

    #[test]
    fn test_pos_5() {
        let mut state: chess::State = str::parse(consts::POS_5).unwrap();
        test_position(&mut state, vec![44, 1486, 62379]);
    }

    #[test]
    fn test_pos_6() {
        let mut state: chess::State = str::parse(consts::POS_6).unwrap();
        test_position(&mut state, vec![46, 2079, 89890]);
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
