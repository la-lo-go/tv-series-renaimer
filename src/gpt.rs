use std::collections::HashMap;

use crate::args::TvSeriesRenaimerArgs;
use crate::files::FolderAndFiles;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GptRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: i64,
    #[serde(rename = "max_tokens")]
    pub max_tokens: i64,
    #[serde(rename = "top_p")]
    pub top_p: i64,
    #[serde(rename = "frequency_penalty")]
    pub frequency_penalty: i64,
    #[serde(rename = "presence_penalty")]
    pub presence_penalty: i64,
    #[serde(rename = "response_format")]
    pub response_format: ResponseFormat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GptResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub index: i64,
    pub message: Message,
    pub logprobs: Value,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

pub fn construct_gpt_request(args: &TvSeriesRenaimerArgs, files: HashMap<String, Vec<String>>) -> GptRequest {
    let files_json = serde_json::to_string(&files).unwrap();

    GptRequest {
        model: args.gtp_model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: r#"I'm going to give you a list of folder and video files paths that are each an episode of a series inside of that folder.\n
                Your job is to return a JSON with the folder containing the names of the original paths and the paths of the formatted names of each episode.\n
                The formated episode path will have this structure: "{rest of the path}/{series name} S{Season Number}E{Episode number} ({year})[{Quality}].{extension}". If the year or quality are not defined in the file name do not made it up and do not put anything in there, dont leave any empty square brackets or parentheses.\n
                The formated Folder Name should be have this structure: "{rest of the path}/S{Season Number}". If you cannot know the season number from the folder original name infer it from the episodes inside.\n
                The seasons and episodes number must be at least two digits long.\n
                Response example:\n
                ```json\n[
                    "{formated Folder Name}":[
                        {
                            "original_path": "{original_path_1}",
                            "formatted_path":  "{formatted_path_1}"
                        },
                        {
                            "original_path": "{original_path_2}",
                            "formatted_path":  "{formatted_path_2}"
                        },
                        ...
                        {
                            "original_path": "{original_path_N-1}",
                            "formatted_path":  "{formatted_path_N-1}"
                        },
                        {
                            "original_path": "{original_path_N}",
                            "formatted_path":  "{formatted_path_N}"
                        }
                    ]
                ]
                ```"#.to_string(),            },
            Message {
                role: "user".to_string(),
                content: files_json.to_string(),
            },
        ],
        temperature: 1,
        max_tokens: 2000,
        top_p: 1,
        frequency_penalty: 0,
        presence_penalty: 0,
        response_format: ResponseFormat {
            type_field: "json_object".to_string(),
        },
    }
}
