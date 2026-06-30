use crate::check;

pub fn get_download() -> String {
    let base_url = "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin";
    let url = format!(
        "{}{}{}",
        base_url,
        check::arch_detection(),
        check::os_detection()
    );
    String::from(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download() {
        let url = get_download();
        let os = crate::check::os_detection();
        let arch = crate::check::arch_detection();

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

        assert_eq!(url, expected_url);
    }
}
