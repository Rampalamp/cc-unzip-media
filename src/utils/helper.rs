use crate::utils::zipperman;
use std::error::Error;
use std::path::{Path, PathBuf};

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
        (1, 1) => zipperman::unzip_pantz(&src_buf, &dest_buf),
        _ => zipperman::unzip_pantz_net(&src, &dest),
    }
}
