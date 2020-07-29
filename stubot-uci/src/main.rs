mod uci;

use std::io;

fn main() -> Result<(), std::io::Error> {
    uci::uci(io::stdin(), io::stdout())?;
    Ok(())
}
