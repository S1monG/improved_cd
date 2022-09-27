use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use clap::builder::ArgAction::SetTrue;

use crate::traversal::get_subdirs_with_name;

mod traversal;

#[derive(Parser,Default,Debug)]
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
    let dirs = get_subdirs_with_name(&args.file_name, &std::env::current_dir().unwrap(), max_depth);

    if dirs.len() == 0 {
        no_dirs(args)
    } else if dirs.len() == 1 {
        one_dir(dirs);
    } else {
        more_than_one_dir(dirs);
    }

}

fn no_dirs(args: Args) {
    eprintln!("No directories with name '{}'", args.file_name);
}

fn one_dir(dirs: Vec<(PathBuf, usize)>) {
    let path_name = dirs.get(0).unwrap().0.to_str().unwrap();
    println!("{}", path_name);
}

fn more_than_one_dir(dirs: Vec<(PathBuf, usize)>) {
    
}
