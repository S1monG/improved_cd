use std::env;
use std::fs;
use std::path::Path;

// make sure that all print statments except the result goes
// to the standard error eprintln!()

fn main() {

    let dir = Path::new("C:/Users/simon");

    let entries = fs::read_dir(dir).unwrap();
    
    for (n, entry) in entries.enumerate() {
        println!("{}  :  {:?}", n, entry);
    }
}
