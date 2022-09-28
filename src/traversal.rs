use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use jwalk::{WalkDir};

pub fn get_subdirs_with_name(
    file_name: &str,
    start: &Path,
    max_depth: usize,
) -> Vec<PathBuf> {

    let mut paths = Vec::new();

    for entry in WalkDir::new(start) {
        //skip directories we dont have access to
        if let Ok(dir) = entry {
            if dir.file_name() == file_name {
                paths.push(dir.path());
            }
        }
    }    

    paths

}
