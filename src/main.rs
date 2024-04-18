mod zipmod;
use std::env;
use zipmod::zipassist;

fn main() {
    match zipassist::try_unzip(env::args().collect()) {
        Ok(_) => println!("Your Pantz Have Been Unzipped!!"),
        Err(e) => {
            e.show_instructions();
            println!(
                "Something Went Wrong While Unzipping Your Pantz : {}",
                e.message
            )
        }
    }
}
