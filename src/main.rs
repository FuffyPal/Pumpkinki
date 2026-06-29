use dotenv::dotenv;
use std::env;
fn os_detection() -> i8 {
    if cfg!(target_os = "windows") {
        1 // windows
    } else if cfg!(target_os = "linux") {
        2 // linux
    } else if cfg!(target_os = "macos") {
        3 // macos
    } else {
        0 // bsd etc.
    }
}

fn find_architecture() -> i8 {
    if cfg!(target_arch = "x86_64") {
        1 // intel & amd
    } else if cfg!(target_arch = "x86") {
        2 // intel & amd but 32-bit
    } else if cfg!(target_arch = "arm") {
        3 // arm 32-bit
    } else if cfg!(target_arch = "aarch64") {
        4 // m1,m2,m3,m4 and arm v8+bit
    } else {
        0 // risc-v etc.
    }
}

fn main() {
    let _ = dotenv();
    let debug: i8 = env::var("DEBUG")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .expect("0,1");
    if debug == 1 {
        println!("OS Code: {}", os_detection());
        println!("Architecture Code: {}", find_architecture());
    }
}