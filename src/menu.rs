use std::collections::HashMap;

use inquire::Confirm;

use crate::files::RetrievedFoldersAndFiles;

pub fn accept_files(files: &HashMap<String, Vec<String>>) -> bool {
    let files_serde = serde_json::to_string_pretty(&files).unwrap();

    accept(
        vec!["\nThese are the files found:".to_string(), files_serde],
        "Do you want to continue?".to_string(),
    )
}

pub fn accept_gpt_response(gpt_response: &RetrievedFoldersAndFiles) -> bool {
    let gpt_response_serde = serde_json::to_string_pretty(&gpt_response)
        .unwrap()
        .replacen("[\n", "", 1)
        .replacen("\n]", "", 1);

    accept(
        vec![
            "\nThis is the GPT response:".to_string(),
            gpt_response_serde,
        ],
        "Do you want to RENAME the files?".to_string(),
    )
}

fn accept(messages: Vec<String>, accept_message: String) -> bool {
    println!("{}", messages.join("\n"));

    Confirm::new(&accept_message)
        .with_default(false)
        .prompt()
        .unwrap()
}
