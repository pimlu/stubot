use super::*;

use std::io::{self, BufRead, Read, Write};

// want to deploy this elsewhere eventually, so streams are generic
pub fn uci(stdin_: impl io::Read, mut stdout: impl io::Write) {
    let w = &mut stdout;
    let mut stdin = io::BufReader::new(stdin_);
    let mut line_buf = String::new();

    // state associated with the IO thread.
    let mut state: chess::State = Default::default();

    while stdin.read_line(&mut line_buf).unwrap() > 0 {
        let line = line_buf.trim();
        let cmd = |name: &str| {
            let full_name = format!("{} ", name);
            if line == name {
                return Some("".to_string());
            } else if line.starts_with(&full_name) {
                Some(line[full_name.len()..].to_string())
            } else {
                None
            }
        };
        let parse_n = |n: String, def| str::parse(n.as_str()).unwrap_or(def);
        if let Some(_) = cmd("uci") {
            writeln!(w, "id name stubot 1.0");
            writeln!(w, "id author Stuart Geipel");
            writeln!(w, "uciok");
        } else if let Some(_) = cmd("debug") {
            // nothing for now
        } else if let Some(_) = cmd("isready") {
            writeln!(w, "readyok");
        } else if let Some(_) = cmd("setoption name") {
            // nothing for now
        } else if let Some(_) = cmd("register") {
            // nothing for now
        } else if let Some(_) = cmd("ucinewgame") {
            state = Default::default();
        } else if let Some(arg) = cmd("position") {
            let parts: Vec<_> = arg.split(" moves ").collect();
            state = match parts[0] {
                "startpos" => Default::default(),
                s => str::parse(s).unwrap()
            };
            if let Some(moves) = parts.get(1) {
                state.run_moves(moves.split(" "));
            }
        } else if let Some(_) = cmd("go") {
            // nothing for now
        } else if let Some(_) = cmd("stop") {
            // nothing for now
        } else if let Some(_) = cmd("quit") {
            std::process::exit(0);
        } else if let Some(moves) = cmd("move") {
            state.run_moves(moves.split(" "));
        } else if let Some(mv_str) = cmd("safe_move") {
            // when unmake_move trashes the state, we can't trust movegen.
            let mut cpy = state.clone();
            if let Some(mv) = cpy.find_move(mv_str.as_str()) {
                state.make_move(mv.mv);
            } else {
                writeln!(w, "no match");
            }
        } else if let Some(count) = cmd("unmove") {
            for _ in 0..parse_n(count, 1) {
                state.unmake_move();
            }
        } else if let Some(_) = cmd("pprint") {
            writeln!(w, "{}", state.board_string());
        }  else if let Some(depth) = cmd("perft") {
            cmds::perftree(&mut state, parse_n(depth, 1));
        } else {
            writeln!(w, "Unknown command: {}", line);
        }
        w.flush().unwrap();
        // read_line appends, clear buffer
        line_buf = String::new();
    }
}
