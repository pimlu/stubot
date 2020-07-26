mod chess;
mod cmds;
mod uci;

use std::io;

fn main() {
    uci::uci(io::stdin(), io::stdout());
}
