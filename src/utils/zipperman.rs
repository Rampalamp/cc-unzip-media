use crate::utils::sshinfo::SSHInfo;
use rar::Archive;
use ssh2::Session;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;
use zip::read::ZipArchive;

pub fn determine_locality_and_unzip(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    //confirm the supplied 2 arguments are a valid path
    let src_buf: PathBuf = PathBuf::from(src);
    if !src_buf.is_absolute() {
        return Err("Invalid Source Path: Not an absolute path".into());
    }

    let dest_buf: PathBuf = PathBuf::from(dest);
    if !dest_buf.is_absolute() {
        return Err("Invalid Destination Path: Not an absolute path".into());
    }

    //split and confirm if either supplied path is over a network.
    let src_parts: Vec<&str> = src.splitn(2, ":").collect();
    let dest_parts: Vec<&str> = dest.splitn(2, ":").collect();

    match (src_parts.len(), dest_parts.len()) {
        (1, 1) => unzip_pantz(&src_buf, &dest_buf),
        _ => unzip_pantz_net(&src, &dest),
    }
}

fn unzip_pantz(src: &PathBuf, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let src_entries = fs::read_dir(src)?;

    for entry in src_entries {
        let entry: fs::DirEntry = entry?;
        let file_type: fs::FileType = entry.file_type()?;
        let src_path: PathBuf = entry.path();
        let dest_path: PathBuf = dest.join(entry.file_name());
        //for some reason even when using continue, it is copying all of the files. in theory only files that are copied are not .zip or .rar.
        if file_type.is_dir() {
            fs::create_dir_all(&dest_path)?;
            unzip_pantz(&src_path, &dest_path)?;
            continue;
        }

        if src_path.extension().map_or(false, |ext| ext == "zip") {
            let folder_name: String = String::from(src.file_name().unwrap().to_str().unwrap());
            println!("Processing ZIP File : {}", folder_name);
            continue;
        }

        if src_path.extension().map_or(false, |ext| ext == "rar") {
            let folder_name: String = String::from(src.file_name().unwrap().to_str().unwrap());
            println!("Processing RAR File : {}", folder_name);
            continue;
        }

        fs::copy(&src_path, &dest_path)?;
    }

    Ok(())
}

fn unzip_pantz_net(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    //parse paths, determine which ones need ssh.

    //parse outhost and
    let host: String = String::from("host");
    let port: i32 = String::from("").parse::<i32>().unwrap();

    match SSHInfo::parse_ssh_info("") {
        Ok(info) => {
            //Need setup this https://docs.rs/ssh2/latest/ssh2/ using ssh_info
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let mut ssh: Session = Session::new()?;

    ssh.handshake()?;

    Ok(())
}
