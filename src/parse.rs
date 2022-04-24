use nipper::{Document, Selection};

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

use crate::widget::content::Project;

lazy_static! {
    static ref RE: Regex = Regex::new(r"<.*>").unwrap();
}

const NA: &str = "N/A";

pub fn parse_category(html: impl AsRef<str>) -> Result<Vec<Project>> {
    let doc = Document::from(html.as_ref());
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
    Ok(projects)
}

/// 不停往前找，找到第一个 h2 就是类别
fn find_category<'a>(pi: &Selection<'a>) -> String {
    if pi.is("h2") {
        return pi.text().to_string();
    }
    find_category(&pi.prev_sibling())
}

fn get_desc<'a>(p: &Selection<'a>) -> String {
    let pc = p
        .html()
        .split("<br>")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let need_replace = pc[1].trim().replace("</p>", "").replace("\n", "");

    RE.replace_all(&need_replace, "").to_string()
}

pub fn parse_volume(html: impl AsRef<str>) -> Result<Vec<Project>> {
    let doc = Document::from(html.as_ref());

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
    Ok(projects)
}

fn get_url<'a>(a: &Selection<'a>) -> String {
    a.attr("href")
        .unwrap()
        .replace("/periodical/statistics/click/?target=", "")
}

pub fn parse_search(html: impl AsRef<str>) -> Result<Vec<Project>> {
    let doc = Document::from(html.as_ref());

    let volume = doc.select("h1").text().to_string();

    let projects: Vec<Project> = doc
        .select(".content-subhead")
        .iter()
        .map(|content| {
            let a = content.select(".project-url");
            let name = a.text().to_string();


            let url = match a.attr("href") {
                Some(href) => {
                    href.replace("/periodical/statistics/click/?target=", "")
                }
                _ => {
                    return None;
                }
            };

            let p = content.next_sibling();

            let p_text = p.text();

            let mut desc_iter = p_text
                .split("\n")
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
            let mut span_text_iter = span_text.split("、");

            let volume = span_text_iter.next().unwrap();

            let category = span_text_iter.next().unwrap();

            Some(Project::new(
                name,
                volume.to_string(),
                category.to_string(),
                url,
                desc.to_string(),
                star.to_string(),
                NA.to_string(),
                NA.to_string(),
            ))
        })
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect();

    Ok(projects)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_search() {
        let html = include_str!("../search.html");
        let projects = parse_search(html).unwrap();
        assert_eq!(10, projects.len());
    }

    #[test]
    fn test_parse_volume() {
        let html = include_str!("../volume.html");
        let projects = parse_volume(html).unwrap();
        assert_eq!(26, projects.len());
    }

    #[test]
    fn test_parse_category() {
        let html = include_str!("../category.html");
        let projects = parse_category(html).unwrap();
        println!("{:?}", projects);
    }
}
