mod chess;
mod types;

fn main() {
    let board: chess::State = chess::State::init_board();
    println!("{}", board);
}
