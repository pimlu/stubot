mod chess;
mod cmds;

use std::env;
use std::io;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let depth: u32 = str::parse(&args[1]).unwrap();
    let mut state: chess::State = str::parse(&args[2]).unwrap();
    let moves_str: Vec<&str> = args[3].split(" ").collect();

    'outer: for mv_str in moves_str {
        if mv_str.is_empty() {
            continue;
        }
        for meta in state.get_moves() {
            if meta.mv.to_string() == mv_str {
                state.make_move(meta.mv);
                continue 'outer;
            }
        }
        panic!("no matching move");
    }

    let mut sum: u64 = 0;
    let mut moves: Vec<_> = state
        .get_moves()
        .iter()
        .map(|meta| {
            let mv = meta.mv;
            let nodes = cmds::perft(&mut state, depth - 1);
            sum += nodes;
            (mv.to_string(), nodes)
        })
        .collect();

    moves.sort_by_key(|tup| tup.0.to_string());

    for (mv, n) in moves {
        println!("{} {}", mv, n);
    }
    println!("");
    println!("{}", sum);
}
