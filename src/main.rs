mod chess;

fn main() {
    let mut state: chess::State = Default::default();
    let moves = state.get_moves();
    assert_eq!(moves.len(), 20);
    println!("{}", state.board_string());
}

#[cfg(test)]
mod test;
