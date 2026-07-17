use std::fs;

pub fn os_detection() -> String {
    if cfg!(windows) {
        return String::from("-Windows.exe");
    } else if cfg!(target_os = "macos") {
        return String::from("-macOS");
    } else if cfg!(target_os = "linux") {
        return String::from("-Linux");
    } else {
        return String::from("other");
    }
}

pub fn arch_detection() -> String {
    if cfg!(target_arch = "x86_64") {
        return String::from("-X64");
    } else if cfg!(target_arch = "aarch64") {
        return String::from("-ARM64");
    } else {
        return String::from("other");
    }
}

pub fn do_you_have_any_pumpkin(path: &String, base: &String) -> i8 {
    let full_name = format!("{}{}{}{}", path, base, arch_detection(), os_detection(),);
    match fs::exists(full_name) {
        Ok(true) => 0,
        _ => 1,
    }
}
