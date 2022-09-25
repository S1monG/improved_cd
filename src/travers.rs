use std::env;
use std::hash::Hash;
use std::path::Path;
use std::collections::HashSet;
use std::fs::{self, DirEntry};

// paths are returned with double forward slashes
// for example "C:\\Users\\simon\\Programming\\Rust\\improved_cd\\target"


// goes through every subdirectory recursevly and finds the dir(s) which match the file_name
fn all_subdirs(start: &Path) -> HashSet<&Path> {
    let sub_dirs = get_sub_dirs(&start);
    if sub_dirs.is_empty() {
        let res = HashSet::new();
        res.insert(start);
        res
    } else {
        sub_dirs.flat_map(|s| all_subdirs(s.unwrap().path()))
    }
}

fn find_target(file_name: String, rd: std::fs::ReadDir) -> std::fs::ReadDir {
    let res = rd.filter(|dir| dir.unwrap().file_name().to_str().unwrap() == file_name);
    HashSet::from(res)
}

fn get_sub_dirs(path: &Path) -> HashSet<&Path> {
    let entries = fs::read_dir(&path).unwrap();
    let entries = entries.filter(|p| is_directory(&p.unwrap().path()));
    let entries = entries.map(|p| &p.unwrap().path());
    HashSet::from_iter(entries)
}

fn is_directory(path: &Path) -> bool {
    fs::metadata(&path).unwrap().is_dir()
}