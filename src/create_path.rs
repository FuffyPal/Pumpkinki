use std::fs;

pub fn create_path(path: String) -> i8 {
    match fs::create_dir_all(path) {
        Ok(()) => return 0,
        Err(_) => return 1,
    }
}
