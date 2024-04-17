use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ZIPError {
    pub message: String,
}

impl ZIPError {
    pub fn new(message: &str) -> Self {
        ZIPError {
            message: message.to_string(),
        }
    }

    pub fn show_instructions(&self) {
        println!("Oops! Looks like your zipper is stuck, don't fuss or do anything sudden, maybe we can get through this unharmed.");
        println!(
            "I am expecting <Source> absolute path, followed by a <Destination> absolute path..."
        );
        println!("ie. 'cc-unzip-media /home/user/doc/src /home/user/doc/dest'");
        println!("You can also preceed both the <Source> and <Destination> paths with the '-n' flag for transfers over network.");
        println!("You will be prompted for network credentials.");
    }
}

impl fmt::Display for ZIPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ZIPError {}
