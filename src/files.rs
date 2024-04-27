use glob::{glob, Paths};
use regex::Regex;
use serde::Serialize;
use std::{collections::HashMap, iter::Chain};

#[derive(Debug, PartialEq, Serialize)]
pub struct FolderAndFiles {
    pub entries: HashMap<String, Vec<String>>,
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

    for video_path in videos_paths {
        if let Ok(video_path) = video_path {
            if let Some(folder) = video_path.parent().and_then(|p| p.to_str()) {
                let normalized_folder = re.replace_all(&folder, "/").to_string();
                let entry = folders.entry(normalized_folder).or_insert_with(Vec::new);

                if let Some(video_path_str) = video_path.to_str() {
                    let normalized_path = re.replace_all(&video_path_str, "/").to_string();
                    entry.push(normalized_path);
                }
            }
        }
    }

    FolderAndFiles {
        entries: folders,
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
}
