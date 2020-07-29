use std::env;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let depth: u32 = str::parse(&args[1]).unwrap();
    let mut state: chess::State = str::parse(&args[2]).unwrap();

    let mut moves_src = "".to_string();
    if args.len() >= 4 {
        moves_src = args[3].to_string();
    }

    state.run_moves(moves_src.split(" "));

    println!("{}", state.perftree(depth));
}
