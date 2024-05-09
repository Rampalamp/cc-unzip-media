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
        for _ in 0..3 {
            println!();
        }
        println!("Oops! Looks like your zipper is stuck, don't fuss or do anything sudden, maybe we can get through this unharmed.");
        println!(
            "I am expecting a <Source> absolute path, followed by a <Destination> absolute path..."
        );
        println!("ie. 'cc-unzip-media /home/user/doc/src /home/user/doc/dest'");
        println!("You can also preceed both the <Source> and <Destination> paths with the '-n' flag for transfers over network.");
        println!("ie. 'cc-unzip-media -n /home/user/doc/src -n /home/user/doc/dest'");
        println!(
            "You will be prompted for network credentials depending which path requires them."
        );
        for _ in 0..3 {
            println!();
        }
    }
}

impl Error for ZIPError {}

impl fmt::Display for ZIPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for ZIPError {
    fn from(error: std::io::Error) -> Self {
        ZIPError::new(&error.to_string())
    }
}
