// We are using the JSON mode of the API so if the word JSON is not in the prompt the API will return an error.

pub const EPISODES_RENAMER: &str = r#"I'm going to give you a list of folder and video files paths that are each an episode of a series inside of that folder.\n
Your job is to return a JSON with the folder containing the names of the original paths and the paths of the formatted names of each episode.\n
The formated episode path will have this structure: "{rest of the path}/{series name} S{Season Number}E{Episode number} ({year})[{Quality}].{extension}". If the year or quality are not defined in the file name do not made it up and do not put anything in there, dont leave any empty square brackets or parentheses.\n
The formated Folder Name should be have this structure: "{rest of the path}/S{Season Number}". If you cannot know the season number from the folder original name infer it from the episodes inside.\n
The seasons and episodes number must be at least two digits long.\n
Response example:\n
```json\n
{
    "folders": [
    {
        "original_folder_name": "{original_folder_name 1}",
        "new_folder_name": "{new_folder_name 1}",
        "files": [
            {
                "original_file_path": "{original_file_path_1_1}",
                "new_file_path": "{formatted_path_1_1}"
            },
            {
                "original_file_path": "{original_file_path_1_2}",
                "new_file_path": "{formatted_path_1_2}"
            },
            ...
        ]
    },
    {
        "original_folder_name": "{original_folder_name 2}",
        "new_folder_name": "{new_folder_name 2}",
        "files": [
            {
                "original_file_path": "{original_file_path_2_1}",
                "new_file_path": "{new_file_path_2_1}"
            },
            {
                "original_file_path": "{original_file_path_2_2}",
                "new_file_path": "{new_file_path_2_2}"
            },
            ...
        ]
    },
    ...
]}
```"#;
