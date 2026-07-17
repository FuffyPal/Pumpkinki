mod check;
mod cli;
mod create_path;
mod url_generation;

use crate::cli::{Cli, Commmand};
use clap::Parser;

fn install(path: String, base: String, os: String, arch: String) {
    if check::do_you_have_any_pumpkin(&path, &base) == 0 {
        println!("this exitis")
    } else if create_path::create_path(&path) == 0 {
        let url = url_generation::get_download(os, arch);
        println!("{}", url);
    } else {
        panic!("Dont Create PATH:  {}", path)
    }
}
fn main() {
    let base: String = String::from("pumpkin");
    let path: String = String::from("./PumpkinMC");
    let args = Cli::parse();
    match args.full {
        Commmand::Install { check, os, arch } => {
            if check {
                let autoos = crate::check::os_detection();
                let autoarch = crate::check::arch_detection();
                install(path, base, autoos, autoarch);
            } else {
                install(
                    path,
                    base,
                    os.expect("Where is a OS?"),
                    arch.expect("Where is a ARCHITECTURE"),
                );
            }
        }
    }
}
