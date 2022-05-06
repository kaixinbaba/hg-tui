use std::sync::Mutex;

use anyhow::{bail, Result};
use cached::proc_macro::cached;

use crate::{app::SearchMode, parse::parse_hg_info, widget::content::Category};

use lazy_static::lazy_static;

lazy_static! {
    static ref LOCK: Mutex<()> = Mutex::new(());
}

/// HelloGitHub 路径前缀
const BASE_PATH: &str = "https://hellogithub.com/periodical";

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
pub fn fetch_hg_info() -> (String, String) {
    // let _lock = LOCK.lock().unwrap();
    // let resp = reqwest::blocking::get("https://github.com/521xueweihan/HelloGitHub").unwrap();
    // let star = parse_hg_star(resp.text().unwrap());
    let star = "55.2k".to_string();

    let resp = reqwest::blocking::get("https://hellogithub.com").unwrap();
    let info = parse_hg_info(resp.text().unwrap());
    (star, info)
}

#[cached]
pub fn fetch_volume(volume: usize) -> String {
    let _lock = LOCK.lock().unwrap();
    let resp = reqwest::blocking::get(format!("{}/volume/{:0>2}/", BASE_PATH, volume)).unwrap();

    resp.text().unwrap()
}

#[cached]
pub fn fetch_category(category: Category, page_no: usize) -> String {
    let url = format!(
        "{}/category/{}/?page={}",
        BASE_PATH,
        category.to_zh(),
        page_no
    );
    let _lock = LOCK.lock().unwrap();
    let resp = reqwest::blocking::get(url).unwrap();

    resp.text().unwrap()
}

#[cached]
pub fn search(wait_search: String) -> String {
    let _lock = LOCK.lock().unwrap();
    let resp = reqwest::blocking::get(format!("{}/search?q={}", BASE_PATH, wait_search)).unwrap();

    resp.text().unwrap()
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[ignore]
    fn test_volume() {
        fetch_volume(72);
    }

    #[test]
    #[ignore]
    fn test_category() {
        fetch_category(Category::C, 1);
    }

    #[test]
    #[ignore]
    fn test_search() {
        search("python".to_string());
    }
}
