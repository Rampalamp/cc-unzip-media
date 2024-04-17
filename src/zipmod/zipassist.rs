use super::zipackage::ZIPackage;
use super::ziperror::ZIPError;
use std::path::PathBuf;

//not sure where each function will reside but...
//need to parse the args, try and make Zipackages.
//determine what prompts next if need network creds.
//create ssh2 channels as property on zip object maybe?
//determine which unzip method to call, if all local, or need network
//finally finish the iterator for unzipping and copying files.

pub fn try_unzip(args: Vec<String>) -> Result<(), ZIPError> {
    match validate_args(args) {
        Ok((src, dest)) => Ok(()),
        Err(e) => Err(e),
    }
}

fn validate_args(args: Vec<String>) -> Result<(ZIPackage, ZIPackage), ZIPError> {
    //Everything should start off count of args, whatever the count is determines what we need to check next.
    //5 = 2 paths needing network
    //4 = 2 paths but one needs networking
    // 3 = local
    //anyhting else is an issue.
    let mut srcZIPackage = ZIPackage {
        path: PathBuf::new(),
        host: String::new(),
        port: 0,
        username: String::new(),
        password: String::new(),
    };
    let mut destZIPackage = ZIPackage {
        path: PathBuf::new(),
        host: String::new(),
        port: 0,
        username: String::new(),
        password: String::new(),
    };

    match args.len() {
        5 => {
            if args[1] != "-n" && args[3] != "-n" {
                return Err(ZIPError::new("Flags are not recognized."));
            }
        }
        4 => {
            if args[1] != "-n" && args[2] == "-n" {
                //first no network but second yes
                //if all good, prompt for host for corresponding dir
            } else if args[1] == "-n" && args[2] != "-n" {
                //second no network but first yea.
                //if all good, prompt for host for corresponding dir
            } else {
                //3 arguments supplied but the commands dont line up.
                return Err(ZIPError::new("Argument orientation not recognized..."));
            }
        }
        _ => {
            return Err(ZIPError::new("Argument orientation not recognized..."));
        }
    }

    Ok((
        ZIPackage {
            path: PathBuf::new(),
            host: String::new(),
            port: 0,
            username: String::new(),
            password: String::new(),
        },
        ZIPackage {
            path: PathBuf::new(),
            host: String::new(),
            port: 0,
            username: String::new(),
            password: String::new(),
        },
    ))
}
