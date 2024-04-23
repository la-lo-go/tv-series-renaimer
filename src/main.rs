#[macro_use]

mod args;
mod check;
mod files;

use args::TvSeriesRenaimerArgs;
use clap::Parser;


fn main() {
    let args = TvSeriesRenaimerArgs::parse();

    println!("{:?} \n", args);

    let errors = check::check_args(&args);
    if !errors.is_empty() {
        check::print_errors(errors);
        std::process::exit(1);
    }
    
    let files = files::find_videos(&args.path);
    if files.is_empty() {
        eprintln!("No files found in the path: {}", args.path);
        std::process::exit(1);
    }

    println!("Files: {:?}", files);
}
