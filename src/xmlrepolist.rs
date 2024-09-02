use roxmltree::{Document, ParsingOptions};
use std::error::Error;

pub fn parse_repo_list(text: String) -> Result<Vec<String>, Box<dyn Error>> {
    let opt = ParsingOptions{
        allow_dtd: true,
        ..ParsingOptions::default()
    };

    let doc = roxmltree::Document::parse_with_options(&text, opt)?;

    let svn_urls: Vec<String> = doc.descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "dir")
        .filter_map(|node| node.attribute("href"))
        .map(|value| value.to_string())
        .collect();

    Ok(svn_urls)
}

pub fn get_last_version(xml: String) -> Result<String, Box<dyn Error>> {
    let doc = Document::parse(&xml)?;

    if let Some(version_name_node) = doc.descendants()
        .find(|node| node.tag_name().name() == "version-name") {
        if let Some(version_name) = version_name_node.text() {
            return Ok(version_name.to_string());
        }
    }

    Err("head version not found or empty".into())
}
