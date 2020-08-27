use engine::{EngineMsg, Searcher};

use futures::future::FutureExt;
use futures::prelude::*;

use tokio::{task, time};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;

use std::str::FromStr;
use std::time::Duration;

const TIME_MUL: f64 = 1.0 / 60.0;
const INC_MUL: f64 = 0.97;
const INF_DEPTH: u32 = 999;

pub struct UciState {
    stop: Arc<AtomicBool>,
    job: Option<future::BoxFuture<'static, ()>>,
    cancel: Option<Box<dyn FnOnce()>>,
    position: chess::State,
    tx: mpsc::Sender<EngineMsg>,
}

impl UciState {
    pub fn new(tx: mpsc::Sender<EngineMsg>) -> Self {
        UciState {
            stop: Arc::new(AtomicBool::new(false)),
            job: None,
            cancel: None,
            position: chess::State::default(),
            tx,
        }
    }
}

impl UciState {
    pub async fn stop_job(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel();
        }
        if let Some(job) = self.job.take() {
            job.await
        }
    }
    pub async fn handle_msg(&mut self, msg: EngineMsg) {
        macro_rules! send {
            ($($arg:tt)*) => {
                self.tx.send(EngineMsg::Output(format!($($arg)*))).unwrap()
            }
        }
        let buf = match msg {
            EngineMsg::Input(s) => s,
            EngineMsg::Output(_) => panic!(),
            EngineMsg::Info(info) => {
                return send!("info {}", info);
            }
            EngineMsg::BestMove(mv) => {
                return send!("bestmove {}", mv);
            }
        };

        // remaining data not consumed by the command process
        let mut rem = buf.as_str().trim();
        // kinda like strip_prefix if it was stable
        let mut cmd = |name: &str| {
            if rem.starts_with(name) {
                rem = &rem[name.len()..].trim_start();
                return true;
            }
            false
        };
        fn parse_n<T: FromStr>(n: &str, def: T) -> T {
            str::parse(n).unwrap_or(def)
        }
        if cmd("uci") {
            send!("id name stubot {}", env!("CARGO_PKG_VERSION"));
            send!("id author Stuart Geipel");
            send!("uciok");
        } else if cmd("debug") {
            // nothing for now
        } else if cmd("isready") {
            send!("readyok");
        } else if cmd("setoption name") {
            // nothing for now
        } else if cmd("register") {
            // nothing for now
        } else if cmd("ucinewgame") {
            self.position = Default::default();
        } else if cmd("position") {
            let parts: Vec<_> = rem.split(" moves ").collect();
            self.position = match parts[0] {
                "startpos" => Default::default(),
                s => {
                    assert!(s.starts_with("fen "));
                    str::parse(&s[4..]).unwrap()
                }
            };
            if let Some(moves) = parts.get(1) {
                self.position.run_moves(moves.split(" "));
            }
        } else if cmd("go") {
            self.stop_job().await;

            let mut args = rem.split_ascii_whitespace();
            let turn = self.position.turn().to_string();
            // usually either time or depth is "infinite" but not both
            let inf_time = Duration::from_secs(365 * 24 * 60 * 60);
            let mut time = Default::default();
            let parse_time = |ms: Option<&str>| Duration::from_millis(parse_n(ms.unwrap(), 0));
            let mut depth = INF_DEPTH;

            if rem.is_empty() {
                time = inf_time;
            }
            while let Some(arg) = args.next() {
                if arg == "infinite" {
                    time = inf_time;
                } else if arg == "movetime" {
                    time += parse_time(args.next());
                } else if arg == format!("{}time", turn) {
                    time += parse_time(args.next()).mul_f64(TIME_MUL);
                } else if arg == format!("{}inc", turn) {
                    time += parse_time(args.next()).mul_f64(INC_MUL);
                } else if arg == "depth" {
                    time = inf_time;
                    depth = parse_n(args.next().unwrap(), 0);
                }
            }

            self.stop.store(false, Ordering::Relaxed);

            let (abort_fut, abort_handle) = future::abortable(future::pending::<()>());

            let pos = self.position.clone();
            let mut searcher = Searcher::new(self.stop.clone(), self.tx.clone());
            let job_task = task::spawn_blocking(move || {
                searcher.uci_negamax(pos, depth);
            });

            // abort or timeout, whichever happens first
            let stop = self.stop.clone();
            let cancel_task = task::spawn(time::timeout(time, abort_fut).map(move |_| {
                stop.store(true, Ordering::Relaxed);
            }));

            // job future waits for cancel, (then stop should be true), then joins on job_task
            let job = cancel_task
                .map(|res| {
                    res.unwrap();
                })
                .then(|_| job_task)
                .map(|res| {
                    res.unwrap();
                });

            self.job = Some(Box::pin(job));
            self.cancel = Some(Box::new(move || abort_handle.abort()));
        } else if cmd("stop") {
            self.stop_job().await;
        } else if cmd("quit") {
            std::process::exit(0);
        } else if cmd("move") {
            self.position.run_moves(rem.split(" "));
        } else if cmd("safe_move") {
            // when unmake_move trashes the state, we can't trust movegen much
            let mut cpy = self.position.clone();
            if let Some(mv) = cpy.find_move(rem) {
                send!("{:?}", mv);
                self.position.make_move(mv);
            } else {
                send!("no match");
            }
        } else if cmd("unmove") {
            for i in 0..parse_n(rem, 1) {
                if self.position.move_len() == 0 {
                    send!("out of moves, unmade {}", i);
                    break;
                }
                self.position.unmake_move();
            }
        } else if cmd("pprint") {
            send!("{}", self.position.board_string());
        } else if cmd("perft") {
            send!("{}", self.position.perftree(parse_n(rem, 1)));
        } else {
            send!("Unknown command: {}", rem);
        }
    }
}
