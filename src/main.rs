mod chess;
mod types;

fn main() {
    let board: chess::State = Default::default();
    println!("{}", board);
}
