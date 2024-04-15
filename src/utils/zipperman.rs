use rar::Archive;
use std::fs;
use std::io::{self, Error};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

pub fn unzip_pantz(src: &Path, dest: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry: fs::DirEntry = entry?;
        let file_type: fs::FileType = entry.file_type()?;
        let src_path: PathBuf = entry.path();
        let dest_path: PathBuf = dest.join(entry.file_name());

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
