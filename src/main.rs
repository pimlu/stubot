mod chess;

fn main() {
    let mut board: chess::State = Default::default();
    let moves = board.get_moves();
    assert_eq!(moves.len(), 20);
    println!("{}", board);
}

#[cfg(test)]
mod test;
