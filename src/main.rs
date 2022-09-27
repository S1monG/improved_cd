use std::path::PathBuf;

use clap::Parser;

use crate::traversal::get_subdirs_with_name;

mod traversal;

#[derive(Parser, Default, Debug)]
#[clap(author = "Simon Gustafsson", version, about)]
/// Improved version of the command cd
struct Args {
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

fn main() {
    let args = Args::parse();
    let max_depth = if args.max_depth.is_some() {
        args.max_depth.unwrap()
    } else {
        usize::max_value()
    };
    let dirs = get_subdirs_with_name(
        &args.file_name,
        &std::env::current_dir().unwrap(),
        max_depth,
    );

    if dirs.is_empty() {
        no_dirs(args)
    } else if dirs.len() == 1 {
        one_dir(dirs);
    } else {
        more_than_one_dir(dirs, &args.file_name);
    }
}

fn no_dirs(args: Args) {
    eprintln!("No directories with name '{}'", args.file_name);
}

fn one_dir(dirs: Vec<(PathBuf, usize)>) {
    let path_name = dirs.get(0).unwrap().0.to_str().unwrap();
    println!("{}", path_name);
}

fn more_than_one_dir(dirs: Vec<(PathBuf, usize)>, file_name: &str) {
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
        let path_name = dirs.get(n).unwrap().0.to_str().unwrap();
        println!("{}", path_name);
    } else {
        eprintln!("Outside of range");
    }
}

fn list_dirs(dirs: &[(PathBuf, usize)]) {
    for (i, dir) in dirs.iter().enumerate() {
        eprintln!("[{}] : {}", i, dir.0.to_str().unwrap());
    }
}

fn parse_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
