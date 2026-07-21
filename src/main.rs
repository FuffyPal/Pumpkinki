mod check;
mod cli;
mod create_path;
mod url_generation;

use crate::cli::{Cli, Commmand};
use clap::Parser;
use std::io;

fn install(path: &str, base: &str, os: &str, arch: &str) -> io::Result<()> {
    let exists = check::do_you_have_any_pumpkin(&path, &base)?;
    if exists {
        println!("this exitis");
    } else if create_path::create_path(&path).is_ok() {
        let url = url_generation::get_download(os, arch)?;
        println!("{}", url);
    } else {
        panic!("Dont Create PATH:  {}", path)
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let base = "pumpkin";
    let path = "./PumpkinMC";
    let args = Cli::parse();
    match args.full {
        Commmand::Install { check, os, arch } => {
            if check {
                let autoos = crate::check::os_detection();
                let autoarch = crate::check::arch_detection();
                install(path, base, &autoos, &autoarch)?;
            } else {
                install(
                    path,
                    base,
                    &os.expect("Where is a OS?"),
                    &arch.expect("Where is a ARCHITECTURE"),
                )?;
            }
        }
    }
    Ok(())
}
