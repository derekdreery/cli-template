#[macro_use]
mod config;

use crate::config::Opt;
use ::structopt::StructOpt;

/// A type alias that saves keystrokes when you want to return a `Result<T, anyhow::Error>` or a
/// `Result<(), anyhow::Error>`.
type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn run(opt: config::Opt) -> Result {
    // your code here.
    log::info!("All done");
    Ok(())
}

main!(run);
