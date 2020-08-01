mod uci;

use uci::*;

use engine::EngineMsg;

use futures::{SinkExt, StreamExt};
use tokio::io;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

use std::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = FramedRead::new(io::stdin(), LinesCodec::new());
    let mut output = FramedWrite::new(io::stdout(), LinesCodec::new());

    let (tx, rx) = mpsc::channel::<EngineMsg>();
    let tx_io = tx.clone();

    let mut uci = UciState::new(tx.clone());

    // subtask to read lines
    tokio::spawn(async move {
        while let Some(line) = input.next().await {
            tx_io.send(EngineMsg::Input(line.unwrap())).unwrap();
        }
    });

    while let Ok(msg) = rx.recv() {
        if let EngineMsg::Output(s) = msg {
            output.send(s).await?;
            continue;
        }
        uci.handle_msg(msg).await;
    }

    Ok(())
}
