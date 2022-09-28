use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

pub fn get_subdirs_with_name(
    file_name: &str,
    start: &Path,
    max_depth: usize,
) -> Vec<(PathBuf, usize)> {
    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    // start with current dir
    queue.push_back((PathBuf::from(start), 0));
    loop {
        if queue.is_empty() {
            break;
        }

        let (entry, depth) = queue.pop_back().unwrap();
        if depth > max_depth {
            continue;
        }

        let dirs = fs::read_dir(entry);
        //skip directories where access is denied
        if let Ok(d) = dirs {
            for dir in d.par_bridge() {
                let dir = dir.unwrap();
                if dir.file_type().unwrap().is_dir() {
                    if dir.file_name() == file_name {
                        paths.push((dir.path(), depth + 1));
                    }
                    queue.push_back((dir.path(), depth + 1));
                }
            }
        }
    }

    paths
}
