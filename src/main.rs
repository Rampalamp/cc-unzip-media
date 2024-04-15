mod utils;
use std::env;
use std::path::{Path, PathBuf};
use utils::zipperman;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <src> <dest>", args[0]);
        return;
    }

    let src_path: &Path = Path::new(&args[1]);
    let dest_path: &Path = Path::new(&args[2]);

    match zipperman::unzip_pantz(src_path, dest_path) {
        Ok(_) => println!("Your Pantz Have Been Unzipped!!"),
        Err(e) => println!("Something Happened While Unzipping Pantz : {:?}", e),
    }
}
