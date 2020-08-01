use super::*;

use chess::{Move, State};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct Searcher {
    pub stop: Arc<AtomicBool>,
    pub nodes: u128,
    pub tx: mpsc::Sender<EngineMsg>,
    // misses checkmates at depth 0, but way faster
    pub negamax_hack: bool,
}

impl Searcher {
    pub fn new(stop: Arc<AtomicBool>, tx: mpsc::Sender<EngineMsg>) -> Self {
        Searcher {
            stop,
            nodes: 0,
            tx,
            negamax_hack: true,
        }
    }
    pub fn uci_negamax(&mut self, mut state: State, depth: u32) {
        self.nodes = 0;
        let start = Instant::now();

        let mut best_mv = None;
        for d in 1..=depth {
            let (mv, score) = self.negamax(&mut state, d);
            if self.stop.load(Ordering::Relaxed) {
                break;
            }
            let el = start.elapsed();
            let nps = Duration::from_secs(1).as_micros() * self.nodes / start.elapsed().as_micros();
            self.tx
                .send(EngineMsg::Info(UciInfo {
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
        self.tx.send(EngineMsg::BestMove(best_mv.unwrap())).unwrap();
    }
    fn negamax(&mut self, state: &mut State, depth: u32) -> (Option<Move>, i16) {
        self.nodes += 1;
        if depth == 0 || self.stop.load(Ordering::Relaxed) {
            let score = if self.negamax_hack {
                state.fast_score()
            } else {
                state.slow_score()
            };
            return (None, state.turn().score(score));
        }
        let mut moves = Vec::with_capacity(state.move_count());
        state.add_sudo_moves(&mut |mv| moves.push(mv));
        let mut best_move = None;
        let mut best_score = None;
        for mv in moves {
            state.make_move(mv);
            if state.is_legal() {
                let score = Some(-self.negamax(state, depth - 1).1);
                if score > best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
            }
            state.unmake_move();
        }
        (best_move, best_score.unwrap_or_else(|| state.slow_score()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const HORIZON_QUEEN: &str = "r1bk1b1r/ppppqppp/5n2/4B3/8/2N5/PPP1QPPP/R3KBNR w KQ - 3 9";
    const ROOK_MATE: &str = "5k2/8/5K1R/8/8/8/8/8 w - - 0 1";

    fn searcher() -> Searcher {
        let (tx, _) = mpsc::channel::<EngineMsg>();
        Searcher::new(Default::default(), tx)
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
            let (mv, _sc) = search.negamax(&mut state, depth - i);
            pv.push(mv.unwrap());
        }
        format!("{}", chess::show_iter(|mv| mv.to_string(), " ", pv))
    }

    #[test]
    #[ignore]
    // at a depth of 5, this can force a queen capture right at the horizon
    fn horizon_queen() {
        assert_eq!(get_pv(HORIZON_QUEEN, 5), "c3d5 f6d5 e5c7 d8c7 e2e7");
    }
    // mate in one, score check
    #[test]
    fn rook_mate() {
        let mut search = searcher();
        let mut pos: State = str::parse(ROOK_MATE).unwrap();
        let (mv, sc) = search.negamax(&mut pos, 3);
        assert_eq!(sc, chess::CHECKMATE);
        assert_eq!(mv.unwrap().to_string(), "h6h8");
    }
}
