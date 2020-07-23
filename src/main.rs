mod chess;

fn main() {
    let board: chess::State = Default::default();
    println!("{}", board);
}
