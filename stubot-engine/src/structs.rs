use chess::{Move, CHECKMATE, MATE_BOUND};

use core::fmt;

use alloc::string::*;
use alloc::vec::Vec;

pub type FoundMv = (Option<Move>, i16);

pub enum EngineMsg {
    Input(String),
    Output(String),
    Info(UciInfo),
    BestMove(Move),
}

pub struct UciInfo {
    pub depth: i32,
    pub score: i16,
    pub nodes: u128,
    pub nps: u128,
    pub time: u128,
    pub pv: Vec<Move>,
}

impl fmt::Display for UciInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let UciInfo {
            depth,
            score,
            nodes,
            nps,
            time,
            pv,
        } = self;
        let score_str = if score.abs() >= MATE_BOUND {
            let base_ply = score - if *score > 0 { CHECKMATE } else { -CHECKMATE };
            // ply offset is negative, round up moves
            format!("mate {}", -(base_ply + base_ply % 2) / 2)
        } else {
            format!("cp {}", score)
        };
        write!(
            f,
            "depth {} score {} nodes {} nps {} time {} pv {}",
            depth,
            score_str,
            nodes,
            nps,
            time,
            chess::show_iter(|mv| mv.to_string(), " ", pv)
        )
    }
}
