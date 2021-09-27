#![no_std]
#[cfg(test)]
extern crate std;
#[macro_use]
extern crate alloc;

mod board;
mod eval;
mod perft;
mod rules;
mod structs;

use board::*;
use eval::*;

pub use board::{show_iter, State};
pub use eval::{mate_ply, CHECKMATE, DRAW, MATE_BOUND};
pub use perft::Perft;
pub use structs::*;
