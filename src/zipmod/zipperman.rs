use super::zipackage::ZIPackage;
use super::ziperror::ZIPError;
use rar::Archive;
use ssh2::Session;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;
use zip::read::ZipArchive;

pub fn determine_locality_and_unzip(src: ZIPackage, dest: ZIPackage) -> Result<(), ZIPError> {
    //check out the ZIPAckages, see which ones need ssh channels
    //if none needed, can just call unzip pantz,
    //if any ssh needed, unzip_pantz_net

    Ok(())
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
        //consider using unrar package instead of rar?
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

    //Need setup this https://docs.rs/ssh2/latest/ssh2/ using ssh_info

    let mut ssh: Session = Session::new()?;

    ssh.handshake()?;

    Ok(())
}
