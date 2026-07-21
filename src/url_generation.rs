use std::io;

pub fn get_download(os: &str, arch: &str) -> io::Result<String> {
    let base_url = "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin";
    let url = format!("{}{}{}", base_url, arch, os);
    Ok(String::from(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_linux_x64() -> io::Result<()> {
        let url = get_download("-Linux", "-X64")?;
        assert_eq!(
            url,
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Linux"
        );
        Ok(())
    }

    #[test]
    fn test_get_download_linux_aarch64() -> io::Result<()> {
        let url = get_download("-Linux", "-ARM64")?;
        assert_eq!(
            url,
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-Linux"
        );
        Ok(())
    }

    #[test]
    fn test_get_download_windows_x64() -> io::Result<()> {
        let url = get_download("-Windows.exe", "-X64")?;
        assert_eq!(
            url,
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Windows.exe"
        );
        Ok(())
    }

    #[test]
    fn test_get_download_windows_aarch64() -> io::Result<()> {
        let url = get_download("-Windows.exe", "-ARM64")?;
        assert_eq!(
            url,
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-Windows.exe"
        );
        Ok(())
    }

    #[test]
    fn test_get_download_macos_aarch64() -> io::Result<()> {
        let url = get_download("-macOS", "-ARM64")?;
        assert_eq!(
            url,
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-macOS"
        );
        Ok(())
    }

    #[test]
    fn test_get_download_macos_x64() -> io::Result<()> {
        let url = get_download("-macOS", "-X64")?;
        assert_eq!(
            url,
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-macOS"
        );
        Ok(())
    }
}
