use std::path::PathBuf;
use std::fs::File;

mod get_file;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let file_name: PathBuf = get_file::get_file();
    let file: File = File::open(file_name)?;

    Ok(())
}
