use std::sync::mpsc;

pub enum LoopMsg {
    Input(String),
    Output(String),
}

pub struct UciState {
    position: chess::State,
}

impl UciState {
    pub fn new() -> Self {
        UciState {
            position: chess::State::default(),
        }
    }
}

impl UciState {
    pub fn queue(&mut self, buf: String, tx: mpsc::Sender<LoopMsg>) {
        macro_rules! send {
            ($($arg:tt)*) => {
                tx.send(LoopMsg::Output(format!($($arg)*))).unwrap()
            }
        }
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
        let parse_n = |n, def| str::parse(n).unwrap_or(def);
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
            // nothing for now
        } else if cmd("stop") {
            // nothing for now
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
