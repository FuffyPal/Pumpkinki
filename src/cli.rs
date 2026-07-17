use clap::{ArgAction, Parser, Subcommand};

fn validate_allowed_os(s: &str) -> Result<String, String> {
    let lower_os = s.to_lowercase();
    match lower_os.as_str() {
        "windows" => Ok("-Windows.exe".to_string()),
        "linux" => Ok("-Linux".to_string()),
        "macos" => Ok("-macOS".to_string()),
        "MacOS" => Ok("-macOS".to_string()),
        "other" => Ok("other".to_string()),
        _ => Err("You can only list “windows“, „linux“, and „macos“ operating systems; for others, please enter „other.“".to_string()),
    }
}

fn validate_allowed_arch(s: &str) -> Result<String, String> {
    let lower_arch = s.to_lowercase();
    match lower_arch.as_str() {
        "x86_64" | "amd64" | "x64" | "x86-64" => {
            Ok("-X64".to_string())
        }
        "aarch64" | "arm64" | "m1" | "m2" | "m3" | "m4" | "m5" | "m6" => {
            Ok("-ARM64".to_string())
        }
        "other" => Ok("other".to_string()),
        _ => Err("You can only select the „x64“ or „arm64“ architectures; for all others, you can select „other“!".to_string()),
    }
}

#[derive(Parser)]
#[command(
    name = "Pumpkinki",
    version,
    about = "A CLI management tool for Pumpkin Minecraft servers, handling processes, RCON, and plugins."
)]
pub struct Cli {
    #[command(subcommand)]
    pub full: Commmand,
}

#[derive(Subcommand)]
pub enum Commmand {
    // /// Starts the Pumpkin server in the background (as a daemon)
    // Start {},

    // /// Safely stops the running Pumpkin server
    // Stop {},

    // /// Forcefully terminates (kills) a running Pumpkin server using its PID
    // ForceStop {},
    /// Download and install the latest version of the Pumpkin server from the internet
    Install {
        /// It checks automatically, and if you set this to `false`, you'll need to specify the values using the `--os` and `--arch` options.
        #[arg(short, long, default_value = "true", action = ArgAction::Set)]
        check: bool,

        #[arg(
            long,
            required_if_eq("check", "false"), // check == "false"
            value_parser = validate_allowed_os
        )]
        os: Option<String>,

        #[arg(
            long,
            required_if_eq("check", "false"), // check == "false"
            value_parser = validate_allowed_arch
        )]
        arch: Option<String>,
    },
    // /// Displays the server's status and, if available, its PID number
    // Status,
}
