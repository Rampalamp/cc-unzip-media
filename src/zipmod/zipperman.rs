use super::zipackage::ZIPackage;
use super::ziperror::ZIPError;
use ssh2::Session;
use std::fs::File;
use std::io;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};
use unrar::error::UnrarError;
use unrar::{Archive, CursorBeforeHeader, OpenArchive, Process};
use zip::read::ZipArchive;

pub fn determine_locality_and_unzip(src: ZIPackage, dest: ZIPackage) -> Result<(), ZIPError> {
    if src.host.trim().is_empty() && dest.host.trim().is_empty() {
        match unzip_pantz(&src.path, &dest.path, &mut PathBuf::new()) {
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

fn unzip_pantz(src: &PathBuf, dest: &PathBuf, temp: &mut PathBuf) -> Result<(), ZIPError> {
    if temp.as_os_str().is_empty() {
        let base_temp_dir: PathBuf = env::temp_dir();

        *temp = PathBuf::from(base_temp_dir);
        temp.push("ccunzip_temp_dir");
    }

    if !temp.exists() {
        match fs::create_dir_all(&temp) {
            Ok(_) => println!("ccunzip_temp_dir created : {:?}", temp),
            Err(e) => println!("Failed to create directory: {}", e),
        }
    }

    let src_entries: fs::ReadDir = fs::read_dir(src)?;

    for entry in src_entries {
        let entry: fs::DirEntry = entry?;
        let src_path: PathBuf = entry.path();
        let dest_path: PathBuf = dest.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            fs::create_dir_all(&dest_path)?;
            unzip_pantz(&src_path, &dest_path, temp)?;
            continue;
        }
        //If I wanted to check to see if a file exists, ideally it would be done around here before any unzipping to the temp folder of machine executing program.
        //this film stem check is kinda not the greatest, many subtitles are being ommitted because they have the same name but different extension...
        //Probably need a more thorough approach to checking for duplicates
        //and then in the process_rar_file I likely need to use the List function to grab the file name, then check if they exist in the dest_path already?
        if file_stem_exists(&src_path, &dest_path) {
            println!(
                "Found Existing File Stem SKIPPING... {}",
                dest_path.to_str().unwrap()
            );
            continue;
        }

        match src_path.extension().and_then(|ext| ext.to_str()) {
            Some("zip") => {
                println!(
                    "Processing ZIP File : {}",
                    src_path.file_name().unwrap().to_str().unwrap()
                );
                match process_zip_file(src_path, &temp) {
                    Ok(_) => copy_and_cleanup(&temp, dest)?,
                    Err(e) => println!("Error : {}", e),
                }
            }
            Some("rar") => {
                println!(
                    "Processing RAR File : {}",
                    src_path.file_name().unwrap().to_str().unwrap()
                );
                match process_rar_file(src_path, &temp) {
                    Ok(_) => copy_and_cleanup(&temp, dest)?,
                    Err(e) => println!("Error {}", e),
                }
            }
            _ => {
                if is_media_file(&src_path) {
                    println!(
                        "Processing Media File : {}",
                        src_path.file_name().unwrap().to_str().unwrap()
                    );
                    match copy_with_retries(&src_path, &dest_path, 3, Duration::from_secs(2)) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Failed to copy file: {:?}", e),
                    }
                    //fs::copy(&src_path, &dest_path)?;
                }
            }
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

fn process_zip_file(src_path: PathBuf, dest_path: &PathBuf) -> Result<(), ZIPError> {
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
                    io::copy(&mut file, &mut outfile)?;
                }
            }
            Ok(())
        }
        Err(e) => {
            println!("Error : {}", e);
            Err(ZIPError::new("Error when creating ZipArchive"))
        }
    }
}

fn process_rar_file(src_path: PathBuf, dest_path: &PathBuf) -> Result<(), UnrarError> {
    match Archive::new(src_path.to_str().unwrap()).open_for_processing() {
        Ok(mut archive) => {
            while let Some(header) = archive.read_header()? {
                let file_name = header.entry().filename.clone();
                let outpath = dest_path.join(file_name);
                archive = if header.entry().is_file() {
                    header.extract_to(outpath)?
                } else {
                    header.skip()?
                };
            }
        }
        Err(e) => {
            println!("UnrarError : {}", e)
        }
    }

    Ok(())
}

fn copy_and_cleanup(temp_path: &PathBuf, destination: &PathBuf) -> io::Result<()> {
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
            match copy_with_retries(&path, &dest_path, 3, Duration::from_secs(2)) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to copy file: {:?}", e),
            }
            //fs::copy(&path, &dest_path)?;
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
            match copy_with_retries(&path, &dest_path, 3, Duration::from_secs(2)) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to copy file: {:?}", e),
            }
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
            "srt" | "sub" | "ssa" | "ass" | "vtt" | "smi" => true, // Subtitle extensions
            _ => false,
        }
    } else {
        false
    }
}

fn file_stem_exists(src_path: &PathBuf, dest_path: &PathBuf) -> bool {
    // Get the file stem of the src_path
    let src_stem: Option<&str> = src_path.file_stem().and_then(|s| s.to_str());

    // Get the parent directory of dest_path
    let dest_dir: PathBuf = dest_path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_path_buf();

    // Iterate over the files in the destination directory
    if let Ok(entries) = fs::read_dir(dest_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    // Get the file stem of the current file
                    let dest_stem: Option<String> = path
                        .file_stem()
                        .and_then(|s| s.to_str().map(|s| s.to_lowercase()));
                    // Compare the file stems
                    if src_stem == dest_stem.as_deref() {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn copy_with_retries(
    src: &PathBuf,
    dest: &PathBuf,
    retries: u32,
    delay: Duration,
) -> io::Result<()> {
    for attempt in 0..retries {
        match fs::copy(src, dest) {
            Ok(_) => return Ok(()),
            Err(e) if attempt < retries - 1 => {
                eprintln!(
                    "Attempt {}: Failed to copy file: {}. Retrying in {:?}...",
                    attempt + 1,
                    e,
                    delay
                );
                sleep(delay);
            }
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to copy file after multiple attempts",
    ))
}
