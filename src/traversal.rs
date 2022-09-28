use std::path::PathBuf;

use jwalk::WalkDir;

pub fn get_subdirs_with_name(
    file_name: String,
    start: PathBuf,
    max_depth: usize,
    _closest: bool,
) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for dir in WalkDir::new(start)
        .max_depth(max_depth)
        .into_iter()
        .flatten()
    {
        //skip directories we dont have access to
        if dir.file_name() == &*file_name {
            paths.push(dir.path());
        }
    }

    paths
}
