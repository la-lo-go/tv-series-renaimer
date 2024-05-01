use glob::{glob, Paths};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, iter::Chain, path};

#[derive(Debug, PartialEq, Serialize)]
pub struct FolderAndFiles {
    pub entries: HashMap<String, Vec<String>>,
}

pub type RetrievedFoldersAndFiles = Vec<FormatedStructure>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatedStructure {
    #[serde(rename = "original_folder_name")]
    pub original_folder_name: String,
    #[serde(rename = "new_folder_name")]
    pub new_folder_name: String,
    pub files: Vec<File>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(rename = "original_file_path")]
    pub original_file_path: String,
    #[serde(rename = "new_file_path")]
    pub new_file_path: String,
}

/// Retrieves a list of video file paths from the specified directory and its subdirectories.
///
/// # Arguments
///
/// * `path` - The path to the directory.
///
/// # Returns
///
/// A `FolderAndFiles` vector with the paths of all video files (with .mkv or .mp4 extension)
/// found in the specified directory and its subdirectories.
pub fn find_videos(path: &str) -> FolderAndFiles {
    let path = path.trim_end_matches('/').trim_end_matches('\\');

    let videos_paths = glob(&format!("{}/**/*.mkv", path))
        .unwrap()
        .chain(glob(&format!("{}/**/*.mp4", path)).unwrap());

    organize_videos(videos_paths)
}

fn organize_videos(videos_paths: Chain<Paths, Paths>) -> FolderAndFiles {
    let mut folders: HashMap<String, Vec<String>> = HashMap::new();
    let re = Regex::new(r#"\\+"#).unwrap();

    for video_path in videos_paths.filter_map(|path| path.ok()) {
        if let Some(parent) = video_path.parent() {
            if let Some(folder) = parent.to_str() {
                let normalized_folder = re.replace_all(folder, "/").to_string();
                let entry = folders.entry(normalized_folder).or_insert_with(Vec::new);

                if let Some(video_path_str) = video_path.to_str() {
                    let normalized_path = re.replace_all(video_path_str, "/").to_string();
                    entry.push(normalized_path);
                }
            }
        }
    }

    FolderAndFiles { entries: folders }
}

pub fn rename_files(folders_and_files: RetrievedFoldersAndFiles, path: &str) {
    for folder in folders_and_files {
        for file in folder.files {
            let original_file_path = path::Path::new(path).join(&file.original_file_path);
            let new_file_path = path::Path::new(path).join(&file.new_file_path);

            if let Some(parent) = new_file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            } else {
                eprintln!("Error: The new file path is invalid");
            }

            println!("> Renaming file {:?} -> {:?}...", original_file_path, new_file_path);
            fs::rename(original_file_path, new_file_path).unwrap();
        }

        println!("Files from folder {:?} have been renamed", folder.original_folder_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_videos() {
        let path = "test/";
        let expected_result = FolderAndFiles {
            entries: {
                let mut folder_files = HashMap::new();
                folder_files.insert(
                    "test/files/Mr Robot".to_string(),
                    vec![
                        "test/files/Mr Robot/MrRobot11.mkv".to_string(),
                        "test/files/Mr Robot/MrRobot102.mp4".to_string(),
                    ],
                );
                folder_files.insert(
                    "test/files/Silicon Valley/S01".to_string(),
                    vec!["test/files/Silicon Valley/S01/SV0101.mkv".to_string()],
                );
                folder_files
            },
        };

        let result = find_videos(path);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_rename_files() {
        let folder_and_files = vec![
            FormatedStructure {
                original_folder_name: "files/Mr Robot".to_string(),
                new_folder_name: "files/Mr Robot/S01/".to_string(),
                files: vec![
                    File {
                        original_file_path: "files/Mr Robot/MrRobot11.mkv".to_string(),
                        new_file_path: "files/Mr Robot/S01/Mr Robot S01E01.mkv".to_string(),
                    },
                    File {
                        original_file_path: "files/Mr Robot/MrRobot102.mp4".to_string(),
                        new_file_path: "files/Mr Robot/S01/Mr Robot S01E02.mp4".to_string(),
                    },
                ],
            },
            FormatedStructure {
                original_folder_name: "files/Silicon Valley/S01".to_string(),
                new_folder_name: "files/Silicon Valley/S01".to_string(),
                files: vec![File {
                    original_file_path: "files/Silicon Valley/S01/SV0101.mkv".to_string(),
                    new_file_path: "files/Silicon Valley/S01/Silicon Valley S01E01.mkv".to_string(),
                }],
            },
        ];

        rename_files(folder_and_files.clone(), "test/");

        let new_files = find_videos("test/").entries;
        assert_eq!(
            new_files.get("test/files/Mr Robot/S01"),
            Some(&vec![
                "test/files/Mr Robot/S01/Mr Robot S01E01.mkv".to_string(),
                "test/files/Mr Robot/S01/Mr Robot S01E02.mp4".to_string(),
            ])
        );
        assert_eq!(
            new_files.get("test/files/Silicon Valley/S01"),
            Some(&vec!["test/files/Silicon Valley/S01/Silicon Valley S01E01.mkv".to_string()])
        );

        revert_renaming(folder_and_files, "test/");
    }

    fn revert_renaming(folders_and_files: RetrievedFoldersAndFiles, path: &str){
        for folder in folders_and_files {
            for file in folder.files {
                let original_file_path = path::Path::new(path).join(&file.original_file_path);
                let new_file_path = path::Path::new(path).join(&file.new_file_path);

                fs::rename(new_file_path, original_file_path).unwrap();
            }
        }
    }
}
