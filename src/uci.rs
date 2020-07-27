use super::*;

use std::io::{self, BufRead, Read, Write};

// want to deploy this elsewhere eventually, so streams are generic
pub fn uci(stdin_: impl io::Read, mut stdout: impl io::Write) {
    let mut stdin = io::BufReader::new(stdin_);
    let mut line = String::new();

    let mut state: chess::State = Default::default();

    while stdin.read_line(&mut line).unwrap() > 0 {
        let try_cmd = |name: &str| {
            let full_name = format!("{} ", name);
            if line == name {
                return Some("".to_string());
            }
            if line.starts_with(&full_name) {
                Some(line[full_name.len()..].to_string())
            } else {
                None
            }
        };
        if let Some(arg) = try_cmd("perft") {
            let depth = match arg.as_str() {
                "" => 1,
                n => str::parse(n).ok().unwrap(),
            };
            cmds::perftree(&mut state, depth);
        }
        stdout.flush().unwrap();
    }
}
