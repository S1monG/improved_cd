use crate::traversal::get_subdirs_with_name;
use crate::utils::*;
use clap::Parser;

use std::path::PathBuf;

mod traversal;
mod utils;

fn main() {
    let args = Args::parse();
    let (file_name, start, max_depth, closest) = process_args(args);

    let is_cached = check_contents(&file_name);
    if is_cached.is_some() {
        eprintln!("DEBUGGGGG");
        one_dir(&PathBuf::from(is_cached.unwrap()));
    } else {
        let dirs = get_subdirs_with_name(file_name.clone(), start, max_depth, closest);

        if dirs.is_empty() {
            no_dirs(&file_name)
        } else if dirs.len() == 1 {
            write_path_to_cache(dirs.get(0).unwrap().to_str().unwrap());
            one_dir(dirs.get(0).unwrap());
        } else {
            more_than_one_dir(dirs, &file_name);
        }
    }
}
