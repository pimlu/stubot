use super::*;

use chess::{State, MATE_BOUND};

use core::cmp;

#[cfg(test)]
use alloc::string::*;
use alloc::vec::Vec;

// NEGAMAX_HACK misses checkmates at depth 0, but is way faster
const NEGAMAX_HACK: bool = true;
pub struct Searcher {
    pub nodes: u128,
}

#[derive(Clone, Copy, Debug)]
pub struct SearchParams {
    depth: i32,
    alpha: i16,
    beta: i16,
}

impl SearchParams {
    pub fn new(depth: i32) -> Self {
        SearchParams {
            depth,
            alpha: -i16::MAX,
            beta: i16::MAX,
        }
    }
    fn tick(&self) -> Self {
        SearchParams {
            depth: self.depth - 1,
            alpha: -self.beta,
            beta: -self.alpha,
        }
    }
    fn contains(&self, score: i16) -> bool {
        self.alpha < score && score < self.beta
    }
}

// negate for negamax, increment checkmate ply counter
fn tick_score(enemy_score: i16) -> i16 {
    let score = -enemy_score;
    if enemy_score.abs() >= MATE_BOUND {
        // decrement towards 0
        score - if score > 0 { 1 } else { -1 }
    } else {
        score
    }
}

impl Searcher {
    pub fn new() -> Self {
        Searcher { nodes: 0 }
    }

    pub fn iter_negamax(
        &mut self,
        state: &mut State,
        depth: i32,
        signal: &impl SearcherSignal,
    ) -> FoundMv {
        // use the prev_score from 2 moves ago to avoid turn instability
        let mut _prev_score = state.fast_score();
        let mut best_mv = (None, _prev_score);
        for d in 1..=depth {
            #[cfg(feature = "iterative_deepen")]
            let found_mv = self.aspiration_negamax(state, d, _prev_score, signal);
            #[cfg(not(feature = "iterative_deepen"))]
            let found_mv = self.negamax(state, SearchParams::new(d), signal);

            if signal.should_stop() {
                break;
            }
            signal.send_partial(self.nodes, d, found_mv).unwrap();
            _prev_score = best_mv.1;
            best_mv = found_mv;
        }
        signal.send_best(best_mv).unwrap();
        return best_mv;
    }
    pub fn aspiration_negamax(
        &mut self,
        state: &mut State,
        depth: i32,
        guess: i16,
        signal: &impl SearcherSignal,
    ) -> FoundMv {
        let mut spread = 30;
        let mut params = SearchParams {
            depth,
            alpha: guess.saturating_sub(spread / 2),
            beta: guess.saturating_add(spread / 2),
        };
        loop {
            let found_mv = self.negamax(state, params, signal);
            let score = found_mv.1;
            if params.contains(score) {
                return found_mv;
            }
            spread = spread.saturating_add(spread);
            // search is stable, so we can use 1 here
            let (sub, add) = if score <= params.alpha {
                (spread, 1)
            } else {
                (1, spread)
            };
            params.alpha = score.saturating_sub(sub);
            params.beta = score.saturating_add(add);
        }
    }
    pub fn negamax(
        &mut self,
        state: &mut State,
        mut params: SearchParams,
        signal: &impl SearcherSignal,
    ) -> FoundMv {
        debug_assert!(
            params.alpha < params.beta,
            "a={} >= b={}",
            params.alpha,
            params.beta
        );
        self.nodes += 1;
        if params.depth <= 0 || signal.should_stop() {
            let abs_score = if NEGAMAX_HACK {
                state.fast_score()
            } else {
                state.slow_score()
            };
            return (None, state.rel_neg(abs_score));
        }
        let mut moves = Vec::with_capacity(state.move_count());
        state.add_sudo_moves(&mut |mv| moves.push((mv, 0)));

        // try best moves first by a shallow estimation
        for (mv, our_score) in &mut moves {
            state.make_move(*mv);
            *our_score = state.rel_neg(state.fast_score());
            state.unmake_move();
        }
        moves.sort_by_key(|&(_, sc)| sc);

        let mut best_move = None;
        let mut best_score = None;
        for (mv, _) in moves {
            state.make_move(mv);
            if state.is_legal() {
                // if the move is legal, check if we can raise alpha
                let enemy_score = self.negamax(state, params.tick(), signal).1;
                let our_score = Some(tick_score(enemy_score));
                params.alpha = cmp::max(params.alpha, our_score.unwrap());
                if our_score > best_score {
                    best_score = our_score;
                    best_move = Some(mv);
                }
            }
            state.unmake_move();
            // if the window closed, stop searching - this never triggers
            // calc_mate because we assume beta > alpha initially
            if params.beta <= params.alpha {
                break;
            }
        }
        let calc_mate = || {
            let abs_score = state.end_score();
            state.rel_neg(abs_score)
        };
        (best_move, best_score.unwrap_or_else(calc_mate))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chess::testpos::*;
    use std::*;

    fn get_pv(fen: &str, depth: i32) -> String {
        let pos: State = str::parse(fen).unwrap();
        let mut pv = Vec::new();
        for i in 0..depth {
            let mut state = pos.clone();
            for &mv in &pv {
                state.make_move(mv);
            }
            let (best_mv, _sc) =
                Searcher::new().negamax(&mut state, SearchParams::new(depth - i), &BlockSignal {});
            if let Some(mv) = best_mv {
                pv.push(mv);
            }
        }
        format!("{}", chess::show_iter(|mv| mv.to_string(), " ", pv))
    }
    fn do_search(fen: &str, depth: i32) -> FoundMv {
        let mut pos: State = str::parse(fen).unwrap();
        Searcher::new().negamax(&mut pos, SearchParams::new(depth), &BlockSignal {})
    }
    fn do_asp_search(fen: &str, depth: i32) -> FoundMv {
        let mut pos: State = str::parse(fen).unwrap();
        Searcher::new().iter_negamax(&mut pos, depth, &BlockSignal {})
    }

    #[test]
    #[ignore]
    // at a depth of 5, this can force a queen capture right at the horizon
    fn horizon_queen() {
        assert_eq!(get_pv(HORIZON_QUEEN, 5), "c3d5 f6d5 e5c7 d8c7 e2e7");
    }
    #[test]
    fn mate_in_2() {
        assert_eq!(get_pv(MATE_2_B, 4), "a4c6 e5e6 d2e2");
        let (_, sc) = do_search(MATE_2_B, 4);
        assert_eq!(sc, chess::mate_ply(3));
    }
    // mate in one, score check
    #[test]
    fn rook_mate_as_white() {
        let (mv, sc) = do_search(ROOK_MATE_W, 3);
        assert_eq!(sc, chess::mate_ply(1));
        assert_eq!(mv.unwrap().to_string(), "h6h8");
    }
    #[test]
    fn rook_mate_as_black() {
        let (mv, sc) = do_search(ROOK_MATE_B, 2);
        assert_eq!(sc, chess::mate_ply(1));
        assert_eq!(mv.unwrap().to_string(), "h3h1");
    }
    #[test]
    #[ignore]
    fn aspiration_bf() {
        for d in 1..=6 {
            println!("depth {}", d);
            for pos in &[YOUR_MOVE, KIWIPETE, POS_3, POS_4, POS_5, POS_6, DUB_M8] {
                let negamax_mv = do_search(pos, d);
                let asp_mv = do_asp_search(pos, d);
                if negamax_mv != asp_mv {
                    println!("fen {}", pos);
                    println!(
                        "depth {}: {} vs {}",
                        d,
                        asp_mv.0.unwrap(),
                        negamax_mv.0.unwrap()
                    );
                }
                assert_eq!(negamax_mv.1, asp_mv.1);
            }
        }
    }
}
