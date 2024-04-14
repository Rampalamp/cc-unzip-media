use rar::Archive;
use std::fs;
use std::io::{self, Error};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

pub fn unzip_pantz(src: &Path, dest: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            fs::create_dir_all(&dest_path)?;
            unzip_pantz(&src_path, &dest_path)?;
            continue;
        }

        if src_path.extension().map_or(false, |ext| ext == "zip") {
            let folder_name = src.file_name().unwrap().to_str().unwrap();

            continue;
        }

        if src_path.extension().map_or(false, |ext| ext == "rar") {}
    }
}
