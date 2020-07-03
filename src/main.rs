mod chess;

fn main() {
    let board: chess::State = chess::State::init_board();
    println!("{}", board);
}
