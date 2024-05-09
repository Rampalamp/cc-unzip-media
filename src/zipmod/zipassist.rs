use super::zipackage::ZIPackage;
use super::ziperror::ZIPError;
use super::zipperman::determine_locality_and_unzip;
use std::io::{self, Write};
use std::path::PathBuf;

//not sure where each function will reside but...
//need to parse the args, try and make Zipackages.
//determine what prompts next if need network creds.
//create ssh2 channels as property on zip object maybe?
//determine which unzip method to call, if all local, or need network
//finally finish the iterator for unzipping and copying files.

pub fn try_unzip(args: Vec<String>) -> Result<(), ZIPError> {
    match validate_args(args) {
        Ok((src, dest)) => match determine_locality_and_unzip(src, dest) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn validate_args(args: Vec<String>) -> Result<(ZIPackage, ZIPackage), ZIPError> {
    //Everything should start off count of args, whatever the count is determines what we need to check next.
    //5 = 2 paths needing network
    //4 = 2 paths but one needs networking
    //3 = local
    //anything else is no bueno.
    let mut src_zipackage: ZIPackage = ZIPackage {
        path: PathBuf::new(),
        host: String::new(),
        port: 0,
        username: String::new(),
        password: String::new(),
    };
    let mut dest_zipackage: ZIPackage = ZIPackage {
        path: PathBuf::new(),
        host: String::new(),
        port: 0,
        username: String::new(),
        password: String::new(),
    };
    let mut src_pathbuf: PathBuf = PathBuf::new();
    let mut dest_pathbuf: PathBuf = PathBuf::new();

    match args.len() {
        5 => {
            if args[1] != "-n" && args[3] != "-n" {
                return Err(ZIPError::new("Flags are not recognized."));
            }
            match validate_and_set_paths(&mut src_pathbuf, &mut dest_pathbuf, &args[2], &args[4]) {
                Ok(()) => {}
                Err(e) => return Err(e),
            }
            //Networking needed for src dr
            match get_network_creds("Source") {
                Ok(info) => map_network_creds(&mut src_zipackage, info),
                Err(e) => return Err(e),
            }
            //Networking needed for destination dir
            match get_network_creds("Destination") {
                Ok(info) => map_network_creds(&mut dest_zipackage, info),
                Err(e) => return Err(e),
            }
        }
        4 => {
            if args[1] != "-n" && args[2] == "-n" {
                match validate_and_set_paths(
                    &mut src_pathbuf,
                    &mut dest_pathbuf,
                    &args[1],
                    &args[3],
                ) {
                    Ok(()) => {}
                    Err(e) => return Err(e),
                }
                //Networking needed for destination dir
                match get_network_creds("Destination") {
                    Ok(info) => map_network_creds(&mut dest_zipackage, info),
                    Err(e) => return Err(e),
                }
            } else if args[1] == "-n" && args[2] != "-n" {
                match validate_and_set_paths(
                    &mut src_pathbuf,
                    &mut dest_pathbuf,
                    &args[2],
                    &args[3],
                ) {
                    Ok(()) => {}
                    Err(e) => return Err(e),
                }
                //Networking needed for source dir
                match get_network_creds("Source") {
                    Ok(info) => map_network_creds(&mut src_zipackage, info),
                    Err(e) => return Err(e),
                }
            } else {
                //3 arguments supplied but the commands dont line up.
                return Err(ZIPError::new("Argument orientation not recognized..."));
            }
        }
        3 => {
            match validate_and_set_paths(&mut src_pathbuf, &mut dest_pathbuf, &args[1], &args[2]) {
                Ok(()) => {}
                Err(e) => return Err(e),
            }
        }
        _ => {
            return Err(ZIPError::new("Argument orientation not recognized..."));
        }
    }

    src_zipackage.path = src_pathbuf;
    dest_zipackage.path = dest_pathbuf;

    Ok((src_zipackage, dest_zipackage))
}

fn get_network_creds(path_type: &str) -> Result<Vec<String>, ZIPError> {
    let mut inputs: Vec<String> = Vec::new();
    let mut host: String = String::new();
    let mut port: String = String::new();
    let mut username: String = String::new();
    let mut password: String;

    //host
    loop {
        print!("Enter {} Host: ", path_type);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut host)
            .expect("Failed to read host line.");
        host = host.trim().to_string();
        if !host.is_empty() {
            break;
        }
        println!("Host must not be blank.");
    }

    inputs.push(host);

    //port
    loop {
        print!("Enter {} Port(Default 22 if left empty): ", path_type);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut port)
            .expect("Failed to read port line.");
        port = port.trim().to_string();
        if port.is_empty() {
            port = "22".to_string();
            break;
        };
        match port.parse::<u16>() {
            Ok(_) => break,
            Err(_) => {
                port.clear();
                println!("Invalid Port Number. Please enter a valid port.")
            }
        };
    }
    inputs.push(port);

    //username
    loop {
        print!("Enter {} UserName: ", path_type);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read host line.");
        username = username.trim().to_string();
        if !username.is_empty() {
            break;
        }
        println!("UserName must not be blank.");
    }
    inputs.push(username);

    //password
    password =
        rpassword::prompt_password("Enter ".to_owned() + &path_type.to_owned() + " Password: ")
            .unwrap();
    password = password.trim().to_string();
    inputs.push(password);

    Ok(inputs)
}

fn map_network_creds(obj: &mut ZIPackage, data: Vec<String>) {
    for (index, value) in data.iter().enumerate() {
        match index {
            0 => obj.host = value.clone(),
            1 => obj.port = value.parse::<u16>().unwrap(),
            2 => obj.username = value.clone(),
            3 => obj.password = value.clone(),
            _ => {}
        }
    }
}

fn validate_and_set_paths(
    src_buf: &mut PathBuf,
    dest_buf: &mut PathBuf,
    src: &String,
    dest: &String,
) -> Result<(), ZIPError> {
    *src_buf = PathBuf::from(src);
    *dest_buf = PathBuf::from(dest);

    //running on a windows machine is_absolute() will return false for a linux path, even if its absolute
    //we could check to see if the dest or src has backwards or forward slashes
    //but simply adding has_root() seems to get our validation working
    if (src_buf.is_absolute() || src_buf.has_root())
        && (dest_buf.is_absolute() || dest_buf.has_root())
    {
        Ok(())
    } else {
        Err(ZIPError::new(
            "One of the paths provided did not validate as an absolute path.",
        ))
    }
}
