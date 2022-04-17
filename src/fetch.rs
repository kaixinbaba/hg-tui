use anyhow::Result;

/// HelloGitHub 路径前缀
const BASE_PATH: &'static str = "https://hellogithub.com/periodical";

pub fn get_volume(volume: usize) -> Result<String> {
    let resp = reqwest::blocking::get(format!("{}/volume/{}/", BASE_PATH, volume))?;

    let text = resp.text()?;

    Ok(text)
}

mod test {
    use super::*;

    #[test]
    fn test_volume() {
        assert!(get_volume(72).is_ok());
    }
}
