#[macro_use]

mod args;

use args::TvSeriesRenaimerArgs;
use clap::Parser;

fn main() {
    let args = TvSeriesRenaimerArgs::parse();

    println!("{:?}", args);
}
