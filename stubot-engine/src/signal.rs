use super::*;

#[cfg(feature = "std")]
use {
    std::sync::atomic::{AtomicBool, Ordering},
    std::sync::mpsc,
    std::sync::Arc,
    std::time::{Duration, Instant},
};

#[cfg(not(feature = "std"))]
type MsgError = ();
#[cfg(feature = "std")]
type MsgError = mpsc::SendError<EngineMsg>;

type MsgSend = Result<(), MsgError>;
pub trait SearcherSignal {
    fn should_stop(&self) -> bool;
    fn send_partial(&self, nodes: u128, depth: i32, mv: FoundMv) -> MsgSend;
    fn send_best(&self, best: FoundMv) -> MsgSend;
}
#[derive(Default)]
pub struct BlockSignal {}
impl SearcherSignal for BlockSignal {
    #[inline]
    fn should_stop(&self) -> bool {
        false
    }
    fn send_partial(&self, _nodes: u128, _depth: i32, _mv: FoundMv) -> MsgSend {
        Result::Ok(())
    }
    fn send_best(&self, _best: FoundMv) -> MsgSend {
        Result::Ok(())
    }
}

#[cfg(feature = "std")]
pub struct StdSignal {
    pub stop: Arc<AtomicBool>,
    pub tx: mpsc::Sender<EngineMsg>,
    start: Instant,
}

#[cfg(feature = "std")]
impl StdSignal {
    pub fn new(stop: Arc<AtomicBool>, tx: mpsc::Sender<EngineMsg>) -> StdSignal {
        StdSignal {
            stop,
            tx,
            start: Instant::now(),
        }
    }
}
#[cfg(feature = "std")]
impl SearcherSignal for StdSignal {
    #[inline]
    fn should_stop(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }
    fn send_partial(&self, nodes: u128, depth: i32, mv: FoundMv) -> MsgSend {
        let (best, score) = mv;
        let el = self.start.elapsed();
        let nps = Duration::from_secs(1).as_micros() * nodes / self.start.elapsed().as_micros();
        self.tx.send(EngineMsg::Info(UciInfo {
            depth,
            score,
            nodes,
            nps,
            time: el.as_millis(),
            pv: vec![best.unwrap()],
        }))
    }
    fn send_best(&self, best: FoundMv) -> MsgSend {
        self.tx.send(EngineMsg::BestMove(best.0.unwrap()))
    }
}
