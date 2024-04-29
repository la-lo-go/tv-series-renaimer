
use inquire::Confirm;

use crate::{files::FolderAndFiles, gpt::GptResponse};

pub fn accept_files(files: &FolderAndFiles) -> bool {
    let files_serde = serde_json::to_string_pretty(&files).unwrap();

    accept(vec![
        "These are the files found:".to_string(),
        files_serde,
    ], "Do you want to continue?".to_string())
}

pub fn accept_gpt_response(gpt_response: &GptResponse) -> bool {
    let gpt_response_serde = serde_json::to_string_pretty(&gpt_response).unwrap();
    
    accept(vec![
        "This is the GPT response:".to_string(),
        gpt_response_serde,
    ], 
    "Do you want to proceed AND RENAME the files?".to_string())
}

fn accept(messages: Vec<String>, accept_message: String) -> bool {
    println!("{}", messages.join("\n"));

    Confirm::new(&accept_message).with_default(false).prompt().unwrap()
}