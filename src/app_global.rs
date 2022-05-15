use std::sync::atomic::AtomicBool;

use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

use crate::{fetch::fetch_hg_info, parse::Info, theme::ThemeStyle};

lazy_static! {
    pub static ref HG_INFO: Info = fetch_hg_info();
    pub static ref HEADERS: Vec<&'static str> = vec!["№", "名称", "期数", "分类", "介绍"];
    pub static ref THEME: OnceCell<ThemeStyle> = OnceCell::new();
    pub static ref IS_COLORFUL: AtomicBool = AtomicBool::new(false);
}
