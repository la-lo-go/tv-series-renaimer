use glob::{glob, Paths};
use std::{collections::HashMap, iter::Chain};


// TODO: Add serde support
#[derive(Debug, PartialEq)]
pub struct FolderAndFiles {
    pub folder: String,
    pub files: Vec<String>,
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
pub fn find_videos(path: &str) -> Vec<FolderAndFiles> {
    let path = path.trim_end_matches('/').trim_end_matches('\\');

    let videos_paths = glob(&format!("{}/**/*.mkv", path))
        .unwrap()
        .chain(glob(&format!("{}/**/*.mp4", path)).unwrap());

    organize_videos(videos_paths)
}

fn organize_videos(videos_paths: Chain<Paths, Paths>) -> Vec<FolderAndFiles> {
    let mut folders: HashMap<String, Vec<String>> = HashMap::new();

    for video_path in videos_paths {
        if let Ok(video_path) = video_path {
            if let Some(folder) = video_path.parent().and_then(|p| p.to_str()) {
                let entry = folders.entry(folder.to_string()).or_insert_with(Vec::new);
                
                if let Some(video_path_str) = video_path.to_str() {
                    entry.push(video_path_str.to_string());
                }
            }
        }
    }

    folders.into_iter().map(|(folder, files)| FolderAndFiles { folder, files }).collect()
}

// TODO: Normalize paths to use only forward slashes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_videos() {
        let path = "test/";
        let expected_result = vec![
            FolderAndFiles {
                folder: "test\\files\\Mr Robot".to_string(),
                files: vec![
                    "test\\files\\Mr Robot\\MrRobot11.mkv".to_string(),
                    "test\\files\\Mr Robot\\MrRobot102.mp4".to_string(),
                ],
            },
            FolderAndFiles {
                folder: "test\\files\\Silicon Valley\\S01".to_string(),
                files: vec![
                    "test\\files\\Silicon Valley\\S01\\SV0101.mkv".to_string(),
                ],
            },
        ];

        let result = find_videos(path);
        assert_eq!(result, expected_result);
    }
}
