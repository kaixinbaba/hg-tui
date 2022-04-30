use std::sync::Mutex;

use anyhow::{bail, Result};
use cached::proc_macro::cached;

use crate::{app::SearchMode, parse::VolumeParser, widget::content::Category};

use lazy_static::lazy_static;

lazy_static! {
    static ref LOCK: Mutex<bool> = Mutex::new(false);
}

/// HelloGitHub 路径前缀
const BASE_PATH: &'static str = "https://hellogithub.com/periodical";

pub fn fetch(text: impl Into<String>, mode: SearchMode) -> Result<String> {
    let html = match mode {
        SearchMode::Normal => search(text.into()),
        SearchMode::Volume => {
            if let Ok(volume) = &text.into()[1..].parse::<usize>() {
                fetch_volume(*volume)
            } else {
                bail!("请输入有效的期数大于 0 的数字！")
            }
        }
        SearchMode::Category => {
            // TODO page_no
            fetch_category(Category::try_from(text.into()[1..].to_string()).unwrap(), 1)
        }
    };

    Ok(html)
}

#[cached]
pub fn fetch_volume(volume: usize) -> String {
    let lock = LOCK.lock().unwrap();
    let resp = reqwest::blocking::get(format!("{}/volume/{:0>2}/", BASE_PATH, volume)).unwrap();

    let text = resp.text().unwrap();

    text
}

#[cached]
pub fn fetch_category(category: Category, page_no: usize) -> String {
    let url = format!(
        "{}/category/{}/?page={}",
        BASE_PATH,
        category.to_zh(),
        page_no
    );
    let lock = LOCK.lock().unwrap();
    let resp = reqwest::blocking::get(url).unwrap();

    let text = resp.text().unwrap();

    text
}

#[cached]
pub fn search(wait_search: String) -> String {
    let lock = LOCK.lock().unwrap();
    let resp = reqwest::blocking::get(format!("{}/search?q={}", BASE_PATH, wait_search)).unwrap();

    let text = resp.text().unwrap();

    text
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
        assert!(fetch_volume(72).is_ok());
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
