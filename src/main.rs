use std::env;
use std::fs;
use std::path::Path;

mod travers;

// make sure that all print statments except the result goes
// to the standard error eprintln!()

fn main() -> std::io::Result<()> {

    let entries = fs::read_dir(env::current_dir()?)?;
    
    for (n, entry) in entries.enumerate() {
        let entry = entry?;
        println!("{}: Path: {:?}     FILE NAME: {:?}", n, entry.path(), entry.file_name());
    }

    let entries = fs::read_dir(env::current_dir()?)?;

    for (n, e) in entries.enumerate() {
        let e = e?;
        let md = fs::metadata(e.path())?;
        println!("{}:   {}", n, md.is_dir());
    }



    Ok(())
}
