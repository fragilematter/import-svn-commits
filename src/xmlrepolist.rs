use roxmltree;
use reqwest;
use url::Url;

pub fn get_repo_list(url: Url) -> Option<String> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send();

    match resp {
        Ok(body) => Some(body.text().expect("could not retrieve repo list")),
        Err(_e) => None
    }
}

pub fn parse_repo_list(text: String) {
    let opt = roxmltree::ParsingOptions{
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };

    let result = roxmltree::Document::parse_with_options(&text, opt);

    if result.is_ok() {
        dbg!(result.unwrap());
    }
}
