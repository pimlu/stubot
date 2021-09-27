use super::*;

use chess::{Move, State, MATE_BOUND};

use core::cmp;

#[cfg(test)]
use alloc::string::*;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::time::{Duration, Instant};

// NEGAMAX_HACK misses checkmates at depth 0, but is way faster
const NEGAMAX_HACK: bool = true;
pub struct Searcher<Signal: SearcherSignal> {
    pub nodes: u128,
    pub signal: Signal,
}

pub struct SearchParams {
    depth: u32,
    alpha: i16,
    beta: i16,
}

impl SearchParams {
    pub fn new(depth: u32) -> Self {
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

impl<S: SearcherSignal + Default> Default for Searcher<S> {
    fn default() -> Self {
        Self::new(S::default())
    }
}

impl<S: SearcherSignal> Searcher<S> {
    pub fn new(signal: S) -> Self {
        Searcher { nodes: 0, signal }
    }
    #[cfg(feature = "std")]
    pub fn uci_negamax(&mut self, mut state: State, depth: u32) {
        self.nodes = 0;
        let start = Instant::now();

        let mut best_mv = None;
        for d in 1..=depth {
            let (mv, score) = self.negamax(&mut state, SearchParams::new(d));
            if self.signal.should_stop() {
                break;
            }
            let el = start.elapsed();
            let nps = Duration::from_secs(1).as_micros() * self.nodes / start.elapsed().as_micros();
            self.signal
                .tx_send(EngineMsg::Info(UciInfo {
                    depth: d,
                    score,
                    nodes: self.nodes,
                    nps,
                    time: el.as_millis(),
                    pv: vec![mv.unwrap()],
                }))
                .unwrap();
            best_mv = mv;
        }
        self.signal
            .tx_send(EngineMsg::BestMove(best_mv.unwrap()))
            .unwrap();
    }
    pub fn negamax(&mut self, state: &mut State, mut params: SearchParams) -> (Option<Move>, i16) {
        self.nodes += 1;
        if params.depth == 0 || self.signal.should_stop() {
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
                let enemy_score = self.negamax(state, params.tick()).1;
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
    use std::sync::mpsc;

    const HORIZON_QUEEN: &str = "r1bk1b1r/ppppqppp/5n2/4B3/8/2N5/PPP1QPPP/R3KBNR w KQ - 3 9";
    const MATE_2_B: &str = "6k1/ppp5/8/4K1p1/b4r2/8/3r4/8 b - - 7 39";
    const ROOK_MATE_W: &str = "5k2/8/5K1R/8/8/8/8/8 w - - 0 1";
    const ROOK_MATE_B: &str = "8/8/8/8/7p/5k1r/8/5K2 b - - 0 1";

    fn searcher() -> Searcher<StdSignal> {
        let (tx, _) = mpsc::channel::<EngineMsg>();

        Searcher::new(StdSignal {
            stop: Default::default(),
            tx,
        })
    }
    fn get_pv(fen: &str, depth: u32) -> String {
        let mut search = searcher();
        let pos: State = str::parse(fen).unwrap();
        let mut pv = Vec::new();
        for i in 0..depth {
            let mut state = pos.clone();
            for &mv in &pv {
                state.make_move(mv);
            }
            let (best_mv, _sc) = search.negamax(&mut state, SearchParams::new(depth - i));
            if let Some(mv) = best_mv {
                pv.push(mv);
            }
        }
        format!("{}", chess::show_iter(|mv| mv.to_string(), " ", pv))
    }
    fn do_search(fen: &str, depth: u32) -> (Option<Move>, i16) {
        let mut search = searcher();
        let mut pos: State = str::parse(fen).unwrap();
        search.negamax(&mut pos, SearchParams::new(depth))
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
}
