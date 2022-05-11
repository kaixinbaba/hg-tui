use std::collections::HashMap;

use nipper::{Document, Selection};

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{app::SearchMode, widget::content::Project};

lazy_static! {
    static ref RE: Regex = Regex::new(r"<.*?>").unwrap();
    pub static ref PARSER: HashMap<SearchMode, &'static dyn Parser> = {
        let mut map = HashMap::<SearchMode, &'static dyn Parser>::new();

        map.insert(SearchMode::Normal, &NormalParser {});
        map.insert(SearchMode::Volume, &VolumeParser {});
        map.insert(SearchMode::Category, &CategoryParser {});

        map
    };
}

const NA: &str = "N/A";

#[derive(Debug, Clone)]
pub enum LastParse {
    Search,

    Volume(String),

    Category(String),
}

pub trait Parser: Sync + Send {
    fn parse(&self, html: String) -> Result<(Vec<Project>, LastParse)>;
}

pub struct NormalParser;

impl Parser for NormalParser {
    fn parse(&self, html: String) -> Result<(Vec<Project>, LastParse)> {
        let doc = Document::from(&html);

        let projects: Vec<Project> = doc
            .select(".content-subhead")
            .iter()
            .filter_map(|content| {
                let a = content.select(".project-url");
                let name = a.text().to_string();

                let url = match a.attr("href") {
                    Some(href) => href.replace("/periodical/statistics/click/?target=", ""),
                    _ => {
                        return None;
                    }
                };

                let p = content.next_sibling();

                let p_text = p.text();

                let mut desc_iter = p_text
                    .split('\n')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty());

                let star = desc_iter.next().unwrap_or(NA).replace("Star ", "");
                let mut desc = desc_iter.next().unwrap_or(NA);

                if desc == "中文" {
                    // 再往下找一个
                    desc = desc_iter.next().unwrap_or(NA);
                }

                let span = p.next_sibling();

                let span_text = span.text();
                let mut span_text_iter = span_text.split('、');

                let volume = span_text_iter.next().unwrap();

                let category = span_text_iter.next().unwrap();

                Some(Project::new(
                    name,
                    volume.to_string(),
                    category.to_string(),
                    url,
                    desc.to_string(),
                    star,
                    NA.to_string(),
                    NA.to_string(),
                ))
            })
            .collect();

        Ok((projects, LastParse::Search))
    }
}

pub struct CategoryParser;
impl Parser for CategoryParser {
    fn parse(&self, html: String) -> Result<(Vec<Project>, LastParse)> {
        let doc = Document::from(&html);
        let category = doc.select("h1").text().to_string();
        let projects: Vec<Project> = doc
            .select("h2.content-subhead")
            .iter()
            .map(|pi| {
                let a = pi.select("a.project-url");
                let name = a.text().to_string();
                let url = get_url(&a);

                let p = pi.next_sibling();

                let info_list: Vec<String> = p
                    .select("i.fa")
                    .iter()
                    .map(|i| i.text().to_string())
                    .collect();

                let volume = info_list[0].clone();
                let star = info_list[1].clone().replace("Star ", "");
                let watch = info_list[2].clone().replace("Watch ", "");
                let fork = info_list[3].clone().replace("Fork ", "");

                let desc = get_desc(&p);

                Project::new(name, volume, category.clone(), url, desc, star, watch, fork)
            })
            .collect();
        Ok((projects, LastParse::Category(category)))
    }
}

pub struct VolumeParser;
impl Parser for VolumeParser {
    fn parse(&self, html: String) -> Result<(Vec<Project>, LastParse)> {
        let doc = Document::from(&html);

        let volume = doc.select("h1").text().to_string();
        let projects: Vec<Project> = doc
            .select("a.project-index")
            .iter()
            .map(|pi| {
                let category = find_category(&pi);

                let name = pi.attr("id").unwrap().to_string();

                let a = pi.next_sibling().next_sibling();
                let url = get_url(&a);
                let p = a
                    .next_sibling()
                    .next_sibling()
                    .next_sibling()
                    .next_sibling();

                let info_list: Vec<String> = p
                    .select("i.fa")
                    .iter()
                    .map(|i| i.text().to_string())
                    .collect();

                let star = info_list[0].clone().replace("Star ", "");
                let watch = info_list[1].clone().replace("Watch ", "");
                let fork = info_list[2].clone().replace("Fork ", "");

                let desc = get_desc(&p);
                Project::new(name, volume.clone(), category, url, desc, star, watch, fork)
            })
            .collect();
        Ok((projects, LastParse::Volume(volume)))
    }
}

/// 不停往前找，找到第一个 h2 就是类别
fn find_category(pi: &Selection) -> String {
    if pi.is("h2") {
        return pi.text().to_string();
    }
    find_category(&pi.prev_sibling())
}

fn get_desc(p: &Selection) -> String {
    let pc = p
        .html()
        .split("<br>")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let need_replace = pc[1].trim().replace("</p>", "").replace('\n', "");

    RE.replace_all(&need_replace, "").to_string()
}

fn get_url(a: &Selection) -> String {
    a.attr("href")
        .unwrap()
        .replace("/periodical/statistics/click/?target=", "")
}

#[allow(dead_code)]
pub fn parse_hg_star(html: String) -> String {
    let doc = Document::from(&html);
    doc.select("#repo-stars-counter-unstar").text().to_string()
}

#[derive(Clone, Debug)]
pub struct Info {
    pub max_volume: usize,
    pub project_count: usize,
    pub star: String,
}

/// 返回最大期数
pub fn parse_hg_info(html: String) -> Info {
    let doc = Document::from(&html);

    let text = doc.select(
        "body > div.l-content > div.pricing-tables.pure-g > div:nth-child(2) > div > div > span",
    )
    .text();
    let result = text.trim().split(' ').into_iter().collect::<Vec<&str>>();
    let project_count = result.get(0).unwrap().parse().unwrap();

    let text = doc.select("body > div.l-content > div.pricing-tables.pure-g > div:nth-child(1) > div > div > span").text();
    let result = text.trim().split(' ').into_iter().collect::<Vec<&str>>();
    let max_volume = result.get(0).unwrap().parse().unwrap();

    Info {
        max_volume,
        project_count,
        star: "55.2k".to_string(),
    }
}

#[cfg(test)]
mod test {

    // #[test]
    // #[ignore]
    // fn test_parse_search() {
    //     let html = include_str!("../search.html");
    //     let projects = NormalParser.parse(html.to_string()).unwrap();
    //     assert_eq!(10, projects.0.len());
    // }
    //
    // #[test]
    // #[ignore]
    // fn test_parse_volume() {
    //     let html = include_str!("../volume.html");
    //     let projects = VolumeParser.parse(html.to_string()).unwrap();
    //     assert_eq!(26, projects.0.len());
    // }
    //
    // #[test]
    // #[ignore]
    // fn test_parse_category() {
    //     let html = include_str!("../category.html");
    //     let projects = CategoryParser.parse(html.to_string()).unwrap();
    //     assert_eq!(10, projects.0.len());
    // }
}
