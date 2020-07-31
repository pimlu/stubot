mod uci;

use uci::*;

use tokio::io;

use futures::{SinkExt, StreamExt};

use std::sync::mpsc;

use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = FramedRead::new(io::stdin(), LinesCodec::new());
    let mut output = FramedWrite::new(io::stdout(), LinesCodec::new());

    let (tx, rx) = mpsc::channel::<LoopMsg>();
    let tx_io = tx.clone();

    let mut uci = UciState::new();

    // subtask to read lines
    tokio::spawn(async move {
        while let Some(line) = input.next().await {
            tx_io.send(LoopMsg::Input(line.unwrap())).unwrap();
        }
    });

    while let Ok(msg) = rx.recv() {
        match msg {
            LoopMsg::Input(s) => uci.queue(s, tx.clone()),
            LoopMsg::Output(s) => {
                output.send(s).await?;
                continue;
            }
        };
    }

    Ok(())
}
