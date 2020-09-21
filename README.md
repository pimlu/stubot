# stubot
###### Chess AI in Rust

Past games on lichess: [Here](https://lichess.org/@/stu_bot/all)

This is a chess AI written in Rust. It has a UCI interface and basic time controls, so it can crush you on lichess if you're mediocre like me. It uses alpha-beta pruning and piece-square tables.

TODO list:

 * Zobrist hashing + transposition/PV tables
 * Quiescence search
 * Evaluation including pawn structure, psuedo move count 

 ## Building and running the code

 `cargo run --release`

