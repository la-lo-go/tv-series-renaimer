use crate::args::TvSeriesRenaimerArgs;


pub fn check_args(args: &TvSeriesRenaimerArgs) -> Vec<String> {
    let mut errors = Vec::new();

    if !check_path(&args.path) {
        errors.push(format!("Invalid path: {}", args.path));
    }

    if !check_mode(&args.mode) {
        eprintln!(
            "Invalid mode: {} see --help for more information",
            args.mode
        );
    }

    errors
}

pub fn check_path(path: &str) -> bool {
    let path = std::path::Path::new(path);
    
    return path.exists()
}

pub fn check_mode(mode: &str) -> bool {
    match mode {
        "recursive" => true,
        "single" => true,
        _ => false,
    }
}
