use std::{fs, io};

pub fn os_detection() -> io::Result<String> {
    if cfg!(windows) {
        return Ok(String::from("-Windows.exe"));
    } else if cfg!(target_os = "macos") {
        return Ok(String::from("-macOS"));
    } else if cfg!(target_os = "linux") {
        return Ok(String::from("-Linux"));
    } else {
        return Ok(String::from("other"));
    }
}

pub fn arch_detection() -> io::Result<String> {
    if cfg!(target_arch = "x86_64") {
        return Ok(String::from("-X64"));
    } else if cfg!(target_arch = "aarch64") {
        return Ok(String::from("-ARM64"));
    } else {
        return Ok(String::from("other"));
    }
}

pub fn do_you_have_any_pumpkin(path: &String, base: &String) -> io::Result<bool> {
    let arch = arch_detection()?;
    let os = os_detection()?;
    let full_path = format!("{}/{}{}{}", path, base, arch, os,);
    let exists = fs::exists(full_path)?;
    Ok(exists)
}
