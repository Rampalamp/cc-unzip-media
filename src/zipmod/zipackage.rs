use std::path::PathBuf;

#[derive(Debug)]
pub struct ZIPackage {
    pub path: PathBuf,
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
}
