use std::path::PathBuf;

#[derive(Debug)]
pub struct ZIPackage {
    pub path: PathBuf,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
