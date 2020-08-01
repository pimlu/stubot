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
}

#[allow(dead_code)]
impl Searcher {
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
            return (None, state.turn().score(state.slow_score()));
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
