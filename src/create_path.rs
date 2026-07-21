use std::{fs, io};

pub fn create_path(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}
