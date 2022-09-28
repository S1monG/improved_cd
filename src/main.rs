use clap::Parser;
use crate::traversal::get_subdirs_with_name;
use crate::utils::*;

mod traversal;
mod utils;


fn main() {
    let args = Args::parse();
    let (file_name, start, max_depth, closest) = process_args(args);
    let dirs = get_subdirs_with_name(
        file_name.clone(),
        start,
        max_depth,
        closest,
    );

    if dirs.is_empty() {
        no_dirs(&file_name)
    } else if dirs.len() == 1 {
        one_dir(dirs);
    } else {
        more_than_one_dir(dirs, &file_name);
    }
}

