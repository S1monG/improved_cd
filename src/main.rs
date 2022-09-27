use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use clap::Parser;

use crate::traversal::get_subdirs_with_name;

mod traversal;


// make sure that all print statments except the result goes
// to the standard error eprintln!()
// you should also be able to chose that you want the closest dir with a flag

#[derive(Parser,Default,Debug)]
#[clap(author = "Simon Gustafsson", version, about)]
/// Improved version of the command cd
struct Args {
    file_name: String,
    #[clap(short, long)]
    max_depth: Option<usize>,
    #[clap(short, long)]
    /// automaticly chose the closest directory if there are multiple directories with the same name
    closest: Option<bool>, 
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
        eprintln!("No directories with name '{}'", args.file_name);
    } else if dirs.len() == 1 {
        println!("{}", dirs.get(0).unwrap().0.to_str().unwrap());
    } else {
        more_than_one_dir(dirs);
    }

}

fn more_than_one_dir(dirs: Vec<(PathBuf, usize)>) {
    
}
