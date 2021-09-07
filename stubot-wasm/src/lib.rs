use chess::{Color, Pos};
use chess::{Move, State};
use engine::{EngineMsg, SearchParams, Searcher};

use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::Arc;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WasmSearcher {
    searcher: Searcher,
}

#[wasm_bindgen]
impl WasmSearcher {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmSearcher {
        let stop = Arc::new(AtomicBool::new(false));
        let (tx, _) = mpsc::channel::<EngineMsg>();
        let searcher = Searcher::new(stop.clone(), tx);
        WasmSearcher { searcher }
    }
    pub fn search(&mut self, mut state: WasmState, depth: u32) -> SearchResult {
        let params = SearchParams::new(depth);
        let (mv, score) = self.searcher.negamax(&mut state.state, params);
        SearchResult { score, mv }
    }
    #[wasm_bindgen(getter)]
    pub fn nodes(&self) -> f64 {
        self.searcher.nodes as f64
    }
}

#[wasm_bindgen]
pub struct WasmState {
    state: State,
}
#[wasm_bindgen]
impl WasmState {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: Option<String>) -> WasmState {
        WasmState {
            state: fen.map_or(State::default(), |s| str::parse(&s).unwrap()),
        }
    }
    #[wasm_bindgen(js_name=toString)]
    pub fn to_string(&self) -> String {
        self.state.to_string()
    }
    #[wasm_bindgen(js_name=moveGen)]
    pub fn move_gen(&mut self) -> String {
        let mvs: Vec<_> = self
            .state
            .gen_moves()
            .into_iter()
            .map(|mv| mv.to_string())
            .collect();
        mvs.join(" ")
    }
    #[wasm_bindgen(js_name=makeMove)]
    pub fn make_move(&mut self, mv: String) {
        let real_mv = self.state.find_move(&mv)
            .unwrap_or_else(|| self.state.find_move(&(mv + "q")).unwrap());
        self.state.make_move(real_mv);
    }
    pub fn score(&mut self) -> i16 {
        self.state.slow_score()
    }
    #[wasm_bindgen(js_name=boardString)]
    pub fn board_string(&self) -> String {
        self.state.board_string()
    }
    #[wasm_bindgen(js_name=isWhite)]
    pub fn is_white(&self) -> bool {
        self.state.turn() == Color::White
    }
}

#[wasm_bindgen]
pub struct WasmPos {
    pos: Pos,
}

#[wasm_bindgen]
impl WasmPos {
    #[wasm_bindgen(constructor)]
    pub fn new(y: i8, x: i8) -> WasmPos {
        WasmPos { pos: Pos {y, x} }
    }
    #[wasm_bindgen(js_name=toString)]
    pub fn to_string(&self) -> String {
        self.pos.to_string()
    }
}

#[wasm_bindgen]
pub struct SearchResult {
    pub score: i16,
    mv: Option<Move>,
}
#[wasm_bindgen]
impl SearchResult {
    #[wasm_bindgen(getter)]
    pub fn mv(&self) -> Option<String> {
        self.mv.map(|m| m.to_string())
    }
}

#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// TODO when std thread is ready for wasm
/*

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, TryRecvError::*};
use std::sync::Arc;
use std::thread;

#[wasm_bindgen]
pub struct WasmSearcher {
    stop: Arc<AtomicBool>,
    rx: mpsc::Receiver<EngineMsg>,
}

#[wasm_bindgen]
impl WasmSearcher {
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed)
    }
    pub fn poll(&self) -> String {
        loop {
            match self.rx.try_recv() {
                Ok(EngineMsg::BestMove(best)) => break best.to_string(),
                Ok(_) => (),
                Err(Empty) => break "pending".to_string(),
                Err(Disconnected) => panic!()
            }
        }
    }
}
#[wasm_bindgen]
pub fn make_search(fen: String, depth: u32) -> WasmSearcher {
    let stop = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel::<EngineMsg>();
    let mut searcher = Searcher::new(stop.clone(), tx);
    thread::spawn(move || searcher.uci_negamax(str::parse(&fen).unwrap(), depth));

    WasmSearcher {
        stop,
        rx
    }
}
*/
