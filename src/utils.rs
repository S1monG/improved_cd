use clap::Parser;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Parser, Default, Debug)]
#[clap(author = "Simon Gustafsson", version, about)]
/// Improved version of the command cd
pub struct Args {
    /// Name of directory/directories to search for
    file_name: String,

    #[clap(short, long)]
    /// Max depth to search for the directory
    max_depth: Option<usize>,

    #[clap(short, long, parse(from_occurrences))]
    /// Automaticly chose the closest directory if there are multiple directories with the same name
    closest: usize,

    #[clap(short, long, parse(from_occurrences))]
    /// Searches from the root if true
    root: usize,
}

pub fn process_args(args: Args) -> (String, PathBuf, usize, bool) {
    let max_depth = if args.max_depth.is_some() {
        args.max_depth.unwrap()
    } else {
        usize::max_value()
    };
    let start = if args.root == 0 {
        std::env::current_dir().unwrap()
    } else {
        PathBuf::from("/")
    };
    let closest = args.closest != 0;

    (args.file_name, start, max_depth, closest)
}

pub fn no_dirs(file_name: &str) {
    eprintln!("No directories with name '{}'", file_name);
    println!("{}", std::env::current_dir().unwrap().display())
}

pub fn one_dir(path: &PathBuf) {
    let path_name = path.to_str().unwrap();
    println!("{}", path_name);
}

pub fn more_than_one_dir(dirs: Vec<PathBuf>, file_name: &str) {
    if dirs.len() > 10 {
        eprintln!(
            "There are {} directories with name '{}'",
            dirs.len(),
            file_name
        );
        eprintln!("Do you want to see all? Y/N");
        let input = parse_input();
        if input.trim().to_lowercase() != "y" {
            std::process::exit(0);
        }
    }

    eprintln!("Choose which directory you want to cd to");

    list_dirs(&dirs);
    let input = parse_input();
    let n: usize = input.trim().parse().unwrap();
    if n <= dirs.len() {
        let path_name = dirs.get(n).unwrap().to_str().unwrap();
        println!("{}", path_name);
    } else {
        eprintln!("Outside of range");
    }
}

pub fn list_dirs(dirs: &[PathBuf]) {
    for (i, dir) in dirs.iter().enumerate() {
        eprintln!("[{}] : {}", i, dir.to_str().unwrap());
    }
}

pub fn parse_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

pub fn write_path_to_cache(path: &str) {
    let mut file = OpenOptions::new().append(true).open("cache.txt").unwrap();
    file.write((String::from("\n") + path).as_bytes()).unwrap();
}

// returns the full path of the first matching filename
// None if the file is not in the cache
pub fn check_contents(file_name: &str) -> Option<String> {
    let mut cache = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("cache.txt")
        .unwrap();
    let mut content = String::new();
    cache.read_to_string(&mut content).unwrap();
    let paths = content.split("\n");
    for row in paths {
        let file = row.split("\\").last().unwrap();
        if file_name == file {
            return Option::from(row.to_string());
        }
    }

    None
}
