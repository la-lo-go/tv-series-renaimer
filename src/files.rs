use glob::{glob, Paths};
use std::{collections::HashMap, iter::Chain};

#[derive(Debug)]
pub struct FolderAndFiles {
    pub folder: String,
    pub files: Vec<String>,
}

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

