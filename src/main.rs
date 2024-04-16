mod utils;
use std::env;
use utils::helper;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <src> <dest>", args[0]);
        return;
    }

    match helper::determine_locality_and_unzip(&args[1], &args[2]) {
        Ok(_) => println!("Your Pantz Have Been Unzipped!!"),
        Err(e) => println!("Something Happened While Unzipping Pantz : {:?}", e),
    }
}
