#[macro_use]

mod args;
mod check;
mod files;
mod gpt;
mod menu;
mod prompts;

use args::TvSeriesRenaimerArgs;
use clap::Parser;

fn main() {
    let args = TvSeriesRenaimerArgs::parse();
    print_logo();

    let errors = check::check_args(&args);
    if !errors.is_empty() {
        close_app(1, Some(&errors.join("\n")));
    }

    let files = files::find_videos(&args.path);
    if files.entries.is_empty() {
        close_app(1, Some("No video files found in the specified path"));
    }

    let menu_response = menu::accept_files(&files.entries);
    if !menu_response {
        close_app(0, None);
    }

    let gpt_request =
        gpt::construct_gpt_request(&args, prompts::EPISODES_RENAMER.to_string(), &files.entries);

    let gpt_response = gpt::send_gpt_request(gpt_request, &args.key);
    if gpt_response == gpt::GptResponse::default() {
        close_app(1, None);
    }

    let new_files_structure = gpt::parse_gpt_response(gpt_response);

    let menu_response = menu::accept_gpt_response(&new_files_structure);
    if !menu_response {
        close_app(0, None);
    }

    files::rename_files(new_files_structure, &args.path);
}

fn close_app(code: i32, error_message: Option<&str>) {
    if let Some(msg) = error_message {
        eprintln!("{}", msg);
    }
    println!("Press enter to close the app");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).unwrap();

    std::process::exit(code);
}

fn print_logo() {
    print!(
        r#"
 _____     _____           _           ______            ___  _____                    
|_   _|   /  ___|         (_)          | ___ \          / _ \|_   _|                   
  | |_   _\ `--.  ___ _ __ _  ___  ___ | |_/ /___ _ __ / /_\ \ | | _ __ ___   ___ _ __ 
  | \ \ / /`--. \/ _ \ '__| |/ _ \/ __||    // _ \ '_ \|  _  | | || '_ ` _ \ / _ \ '__|
  | |\ V //\__/ /  __/ |  | |  __/\__ \| |\ \  __/ | | | | | |_| || | | | | |  __/ |   
  \_/ \_/ \____/ \___|_|  |_|\___||___/\_| \_\___|_| |_\_| |_/\___/_| |_| |_|\___|_|   
"#);
}
