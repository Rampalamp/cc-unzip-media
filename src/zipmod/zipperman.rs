use super::zipackage::ZIPackage;
use super::ziperror::ZIPError;
use rar::Archive;
use ssh2::Session;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::iter::Zip;
use std::net::TcpStream;
use std::os::windows::process;
use std::path::PathBuf;
use std::{env, fs};
use zip::read::ZipArchive;

pub fn determine_locality_and_unzip(src: ZIPackage, dest: ZIPackage) -> Result<(), ZIPError> {
    if src.host.trim().is_empty() && dest.host.trim().is_empty() {
        match unzip_pantz(&src.path, &dest.path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        match unzip_pantz_net(src, dest) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

fn unzip_pantz(src: &PathBuf, dest: &PathBuf) -> Result<(), ZIPError> {
    let base_temp_dir = env::temp_dir();

    let mut ccunzip_temp_dir = PathBuf::from(base_temp_dir);

    ccunzip_temp_dir.push("ccunzip_temp_dir");

    if !ccunzip_temp_dir.exists() {
        match fs::create_dir_all(&ccunzip_temp_dir) {
            Ok(_) => println!("ccunzip_temp_dir created : {:?}", ccunzip_temp_dir),
            Err(e) => println!("Failed to create directory: {}", e),
        }
    }

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
            match process_zip_file(src_path, &ccunzip_temp_dir) {
                Ok(path) => {
                    println!("Copying Unzipped Contents And Cleaning Up...");
                    copy_and_cleanup(&ccunzip_temp_dir, dest_path)?
                }
                Err(e) => println!("Error : {}", e),
            }
            continue;
        }
        //consider using unrar package instead of rar?
        if src_path.extension().map_or(false, |ext| ext == "rar") {
            let folder_name: String = String::from(src.file_name().unwrap().to_str().unwrap());
            println!("Processing RAR File : {}", folder_name);
            continue;
        }

        if is_media_file(&src_path) {
            fs::copy(&src_path, &dest_path)?;
        }
    }

    Ok(())
}

fn unzip_pantz_net(src: ZIPackage, dest: ZIPackage) -> Result<(), ZIPError> {
    //parse paths, determine which ones need ssh.

    //parse outhost and
    let host: String = String::from("host");
    let port: i32 = String::from("").parse::<i32>().unwrap();

    //Need setup this https://docs.rs/ssh2/latest/ssh2/ using ssh_info
    let conn_string = format!("{}:{}", src.host, src.port);
    match TcpStream::connect(conn_string) {
        Ok(tcp) => {
            let mut ssh: Session = Session::new().unwrap();

            match ssh.handshake() {
                Ok(_) => {}
                Err(e) => return Err(ZIPError::new(e.message())),
            }
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

fn process_zip_file(src_path: PathBuf, dest_path: &PathBuf) -> Result<PathBuf, ZIPError> {
    //dest_path in this case should be the temp dir created at start of the program
    //src_path should be full path to the actual .zip file...
    let file: File = File::open(&src_path)?;
    match ZipArchive::new(file) {
        Ok(mut archive) => {
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let outpath: PathBuf = dest_path.join(file.name());

                if (*file.name()).ends_with('/') {
                    fs::create_dir_all(&outpath);
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(&p)?;
                        }
                    }
                    let mut outfile = File::create(&outpath)?;
                    io::copy(&mut file, &mut outfile);
                }
            }
            Ok(dest_path.clone())
        }
        Err(e) => {
            println!("Error : {}", e);
            Err(ZIPError::new("Error when creating ZipArchive"))
        }
    }
}

fn copy_and_cleanup(temp_path: &PathBuf, destination: PathBuf) -> io::Result<()> {
    //copy recursively
    for entry in fs::read_dir(&temp_path)? {
        let entry: fs::DirEntry = entry?;
        let path: PathBuf = entry.path();
        let relative_path: &std::path::Path = path.strip_prefix(&temp_path).unwrap();
        let dest_path: PathBuf = destination.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_directory_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    //cleanup
    for entry in fs::read_dir(&temp_path)? {
        let entry: fs::DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

fn copy_directory_recursive(source: &PathBuf, destination: &PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry: fs::DirEntry = entry?;
        let path: PathBuf = entry.path();
        let dest_path: PathBuf = destination.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_directory_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn is_media_file(path: &PathBuf) -> bool {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            "mp4" | "avi" | "mov" | "mkv" | "webm" | "wmv" => true, // Video extensions
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" => true, // Audio extensions
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "svg" => true, // Image extensions
            _ => false,
        }
    } else {
        false
    }
}
