use super::*;

#[cfg(feature = "std")]
use {
    std::sync::atomic::{AtomicBool, Ordering},
    std::sync::mpsc,
    std::sync::Arc,
};

#[cfg(not(feature = "std"))]
type MsgError = ();
#[cfg(feature = "std")]
type MsgError = mpsc::SendError<EngineMsg>;

type MsgSend = Result<(), MsgError>;
pub trait SearcherSignal {
    fn should_stop(&self) -> bool;
    fn tx_send(&self, msg: EngineMsg) -> MsgSend;
}
#[derive(Default)]
pub struct BlockSignal {}
impl SearcherSignal for BlockSignal {
    #[inline]
    fn should_stop(&self) -> bool {
        false
    }
    fn tx_send(&self, _msg: EngineMsg) -> MsgSend {
        Result::Ok(())
    }
}

#[cfg(feature = "std")]
pub struct StdSignal {
    pub stop: Arc<AtomicBool>,
    pub tx: mpsc::Sender<EngineMsg>,
}
#[cfg(feature = "std")]
impl SearcherSignal for StdSignal {
    #[inline]
    fn should_stop(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }
    fn tx_send(&self, msg: EngineMsg) -> MsgSend {
        self.tx.send(msg)
    }
}
