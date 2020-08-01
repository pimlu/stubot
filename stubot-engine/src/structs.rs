use chess::Move;

pub enum EngineMsg {
    Input(String),
    Output(String),
    Info(UciInfo),
    BestMove(Move),
}

pub struct UciInfo {
    pub depth: u32,
    pub score: i16,
    pub nodes: u128,
    pub nps: u128,
    pub time: u128,
    pub pv: Vec<Move>,
}
