use roxmltree;
use reqwest::blocking::Client;
use url::Url;
use rpassword::read_password;
use std::error::Error;
use std::io;

pub fn get_repo_list(
    url: Url, 
    username: Option<String>, 
    use_password: bool
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

   // If use_password is true, prompt the user for a password.
    let password = if use_password {
        print!("Enter password: ");
        io::Write::flush(&mut io::stdout())?;
        Some(read_password()?)
    } else {
        None
    };

    // Prepare the request.
    let request = if let Some(user) = username {
        if let Some(pass) = password {
            client.get(url).basic_auth(user, Some(pass))
        } else {
            client.get(url).basic_auth(user, None::<String>)
        }
    } else {
        client.get(url)
    };

    // Send the request and get the response.
    let response = request.send()?.text()?;

    Ok(response)
}

pub fn parse_repo_list(text: String) -> Result<Vec<String>, Box<dyn Error>> {
    let opt = roxmltree::ParsingOptions{
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };

    let doc = roxmltree::Document::parse_with_options(&text, opt)?;

    let svn_urls: Vec<String> = doc.descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "dir")
        .filter_map(|node| node.attribute("href"))
        .map(|value| value.to_string())
        .collect();

    Ok(svn_urls)
}
