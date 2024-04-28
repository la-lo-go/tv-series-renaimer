#[macro_use]

mod args;
mod check;
mod files;
mod gpt;
mod prompts;

use args::TvSeriesRenaimerArgs;
use clap::Parser;

fn main() {
    let args = TvSeriesRenaimerArgs::parse();

    println!("{:?} \n", args);

    let errors = check::check_args(&args);
    if !errors.is_empty() {
        close_app(1, Some(&errors.join("\n")));
    }

    let files = files::find_videos(&args.path);
    if files.entries.is_empty() {
        close_app(1, Some("No video files found in the specified path"));
    }

    let gpt_request =
        gpt::construct_gpt_request(&args, prompts::EPISODES_RENAMER.to_string(), files.entries);

    let gpt_response = gpt::send_gpt_request(gpt_request, &args.key);

    // save the request to a file named gpt_request.json
    let gpt_response_json = serde_json::to_string_pretty(&gpt_response).unwrap();
    std::fs::write("gpt_response.json", gpt_response_json).unwrap();
}

// add an optional parameter that is the eeror message
fn close_app(code: i32, error_message: Option<&str>) {
    if let Some(msg) = error_message {
        eprintln!("{}", msg);
    }
    println!("Press enter to close the app");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).unwrap();

    std::process::exit(code);
}
