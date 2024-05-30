use crate::args::TvSeriesRenaimerArgs;
use crate::files::RetrievedFoldersAndFiles;
use core::panic;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

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

#[derive(Debug, Deserialize)]
pub struct GptCompleteResponse {
    folders: RetrievedFoldersAndFiles,
}

#[derive(Debug, Deserialize)]
pub struct GptErrorResponse {
    error: GptErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct GptErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub param: Option<String>,
    pub code: String,
}

pub fn construct_gpt_request(
    args: &TvSeriesRenaimerArgs,
    prompt: String,
    files: &HashMap<String, Vec<String>>,
) -> GptRequest {
    let files_json = serde_json::to_string(&files).unwrap();

    GptRequest {
        model: args.gtp_model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: prompt,
            },
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

pub fn send_gpt_request(request: GptRequest, key: &str) -> GptResponse {
    let client = reqwest::blocking::Client::new();

    println!("Sending the request to the API, this may take a moment...");
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/json".to_string())
        .body(serde_json::to_string(&request).unwrap())
        .send();

    let response = match response {
        Ok(res) => match res.text() {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error obtaining the response text: {}", e);
                return GptResponse::default();
            }
        },
        Err(e) => {
            eprintln!("Error sending the request: {}", e);
            return GptResponse::default();
        }
    };

    let response_result: Result<GptResponse, serde_json::Error> = serde_json::from_str(&response);
    match response_result {
        Ok(response) => response,
        Err(_) => {
            let response_error: Result<GptErrorResponse, serde_json::Error> =
                serde_json::from_str(&response);
            match response_error {
                Ok(response_error) => {
                    println!("Error: {}", response_error.error.message);
                    panic!("Error sending the request");
                }
                Err(e) => {
                    println!("Error parsing the response: {}", e);
                    println!("Response content: {}", response);
                    panic!("Error parsing the response");
                }
            }
        }
    }
}

pub fn parse_gpt_response(response: GptResponse) -> RetrievedFoldersAndFiles {
    let response_json = response.choices[0].message.content.to_string();
    let complete_response: GptCompleteResponse =
        serde_json::from_str(&response_json).expect("Error parsing the response");
    complete_response.folders
}

#[cfg(test)]
mod tests {
    use super::*;

    // DISCLAMER: Remove this key before pushing to a public repository
    const KEY: &str = "sk-proj-........";
    const TEST_GPT: &str = "Return a hello world message, in a json"; // the word "json" must be in the prompt

    #[test]
    fn test_gpt_request() {
        let args = TvSeriesRenaimerArgs {
            key: KEY.to_string(),
            path: "./".to_string(),        // this is not used in the test
            mode: "recursive".to_string(), // this is not used in the test
            gtp_model: "gpt-4o".to_string(),
        };

        let files = HashMap::new();
        let prompt = TEST_GPT.to_string();

        let request = construct_gpt_request(&args, prompt, &files);

        let response = send_gpt_request(request, &args.key);
        assert!(response.choices.len() > 0);
    }

    #[test]
    #[should_panic(expected = "Error sending the request")]
    fn test_bad_key_gpt_request() {
        let args = TvSeriesRenaimerArgs {
            key: "sk-proj-error-key".to_string(),
            path: "./".to_string(),        // this is not used in the test
            mode: "recursive".to_string(), // this is not used in the test
            gtp_model: "gpt-4o".to_string(),
        };

        let files = HashMap::new();
        let prompt = "This is a test that will fail becouse is a wrong API key (json)".to_string();

        let request = construct_gpt_request(&args, prompt, &files);

        send_gpt_request(request, &args.key);
    }
}
