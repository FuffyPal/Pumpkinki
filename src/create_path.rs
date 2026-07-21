use std::{fs, io};

pub fn create_path(path: &String) -> io::Result<()> {
    fs::create_dir_all(path)
}
