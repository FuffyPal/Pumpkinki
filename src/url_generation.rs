use std::io;

pub fn get_download(os: String, arch: String) -> io::Result<String> {
    let base_url = "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin";
    let url = format!("{}{}{}", base_url, arch, os);
    Ok(String::from(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download() {
        let os = crate::check::os_detection().unwrap();
        let arch = crate::check::arch_detection().unwrap();

        let expected_url = match (os.as_str(), arch.as_str()) {
            ("-Windows.exe", "-X64") => {
                "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Windows.exe"
            }
            ("-Windows.exe", "-ARM64") => {
                "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-Windows.exe"
            }
            ("-Linux", "-X64") => {
                "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Linux"
            }
            ("-Linux", "-ARM64") => {
                "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-Linux"
            }
            ("-macOS", "-ARM64") => {
                "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-macOS"
            }
            ("-macOS", "-X64") => {
                "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-macOS"
            }
            _ => panic!("Unsupported OS/Arch combination in test"),
        };
        let url = get_download(os, arch).unwrap();

        assert_eq!(url, expected_url);
    }
}
