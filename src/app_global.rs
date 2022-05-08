use lazy_static::lazy_static;

use crate::{fetch::fetch_hg_info, parse::Info};

lazy_static! {
    pub static ref HG_INFO: Info = fetch_hg_info();
    pub static ref HEADERS: Vec<&'static str> = vec!["№", "名称", "期数", "分类", "介绍"];
}
