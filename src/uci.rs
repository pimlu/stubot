use super::*;

use std::io::{self, BufRead, Error};

// want to deploy this elsewhere eventually, so streams are generic
pub fn uci(stdin_: impl io::Read, mut stdout: impl io::Write) -> Result<(), Error> {
    let w = &mut stdout;
    let mut stdin = io::BufReader::new(stdin_);
    let mut line_buf = String::new();

    // state associated with the IO thread.
    let mut state: chess::State = Default::default();

    while stdin.read_line(&mut line_buf).unwrap() > 0 {
        // remaining data not consumed by the command process
        let mut rem = line_buf.trim();
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
            writeln!(w, "id name stubot 1.0")?;
            writeln!(w, "id author Stuart Geipel")?;
            writeln!(w, "uciok")?;
        } else if cmd("debug") {
            // nothing for now
        } else if cmd("isready") {
            writeln!(w, "readyok")?;
        } else if cmd("setoption name") {
            // nothing for now
        } else if cmd("register") {
            // nothing for now
        } else if cmd("ucinewgame") {
            state = Default::default();
        } else if cmd("position") {
            let parts: Vec<_> = rem.split(" moves ").collect();
            state = match parts[0] {
                "startpos" => Default::default(),
                s => {
                    assert!(s.starts_with("fen "));
                    str::parse(&s[4..]).unwrap()
                }
            };
            if let Some(moves) = parts.get(1) {
                state.run_moves(moves.split(" "));
            }
        } else if cmd("go") {
            // nothing for now
        } else if cmd("stop") {
            // nothing for now
        } else if cmd("quit") {
            std::process::exit(0);
        } else if cmd("move") {
            state.run_moves(rem.split(" "));
        } else if cmd("safe_move") {
            // when unmake_move trashes the state, we can't trust movegen much
            let mut cpy = state.clone();
            if let Some(meta) = cpy.find_move(rem) {
                writeln!(w, "{:?}", meta.mv)?;
                state.make_move(meta.mv);
            } else {
                writeln!(w, "no match")?;
            }
        } else if cmd("unmove") {
            for i in 0..parse_n(rem, 1) {
                if state.move_len() == 0 {
                    writeln!(w, "out of moves, unmade {}", i)?;
                    break;
                }
                state.unmake_move();
            }
        } else if cmd("pprint") {
            writeln!(w, "{}", state.board_string())?;
        } else if cmd("perft") {
            cmds::perftree(&mut state, parse_n(rem, 1));
        } else {
            writeln!(w, "Unknown command: {}", rem)?;
        }
        w.flush().unwrap();
        // read_line appends, clear buffer
        line_buf = String::new();
    }

    Ok(())
}
