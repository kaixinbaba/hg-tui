use anyhow::{bail, Result};

use crate::{app::SearchMode, parse::VolumeParser, widget::content::Category};

/// HelloGitHub 路径前缀
const BASE_PATH: &'static str = "https://hellogithub.com/periodical";

pub fn fetch(text: impl Into<String>, mode: SearchMode) -> Result<String> {
    match mode {
        SearchMode::Normal => search(text),
        // TODO page_no
        SearchMode::Volume => {
            if let Ok(volume) = &text.into()[1..].parse::<usize>() {
                fetch_volume(*volume, 1)
            } else {
                bail!("Invalid volume, must be number")
            }
        }
        SearchMode::Category => todo!(),
    }
}

fn fetch_volume(volume: usize, page_no: usize) -> Result<String> {
    let resp = reqwest::blocking::get(format!(
        "{}/volume/{:0>2}/?page={}",
        BASE_PATH, volume, page_no
    ))?;

    let text = resp.text()?;

    Ok(text)
}

fn fetch_category(category: Category, page_no: usize) -> Result<String> {
    let resp = reqwest::blocking::get(format!(
        "{}/category/{}/?page={}",
        BASE_PATH,
        category.to_zh(),
        page_no
    ))?;

    let text = resp.text()?;

    Ok(text)
}

fn search(wait_search: impl Into<String>) -> Result<String> {
    let resp = reqwest::blocking::get(format!("{}/search?q={}", BASE_PATH, wait_search.into()))?;

    let text = resp.text()?;

    Ok(text)
}

mod test {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    fn write_text(text: impl Into<String>, path: impl AsRef<Path>) {
        let mut f = File::create(path).unwrap();

        f.write_all(text.into().as_bytes()).unwrap();
    }

    #[test]
    fn test_volume() {
        assert!(fetch_volume(72, 1).is_ok());
    }

    #[test]
    fn test_category() {
        assert!(fetch_category(Category::C, 1).is_ok());
    }

    #[test]
    fn test_search() {
        assert!(search("python").is_ok());
    }
}
