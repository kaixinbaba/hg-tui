use nipper::Document;

use anyhow::Result;

use crate::widget::content::Project;

const NA: &str = "N/A";

pub fn parse_category(html: impl AsRef<str>) -> Result<Project> {
    let doc = Document::from(html.as_ref());

    todo!()
}

pub fn parse_volume(html: impl AsRef<str>) -> Result<Project> {
    let doc = Document::from(html.as_ref());

    todo!()
}

pub fn parse_search(html: impl AsRef<str>) -> Result<Vec<Project>> {
    let doc = Document::from(html.as_ref());

    let projects: Vec<Project> = doc
        .select(".content-subhead")
        .iter()
        .map(|content| {
            let a = content.select(".project-url");
            let name = a.text().to_string();

            let url = a
                .attr("href")
                .unwrap()
                .replace("/periodical/statistics/click/?target=", "");

            let p = content.next_sibling();

            let p_text = p.text();

            let mut desc_iter = p_text
                .split("\n")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());

            let star = desc_iter.next().unwrap_or(NA).replace("Star ", "");
            let desc = desc_iter.next().unwrap_or(NA);

            let span = p.next_sibling();

            let span_text = span.text();
            let mut span_text_iter = span_text.split("、");

            let volume = span_text_iter.next().unwrap();

            let category = span_text_iter.next().unwrap();

            Project::new(
                name,
                volume.to_string(),
                category.to_string(),
                url,
                desc.to_string(),
                star.to_string(),
                NA.to_string(),
                NA.to_string(),
            )
        })
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
}
