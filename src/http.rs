use reqwest::{blocking::Client, header::CONTENT_TYPE, Method};
use secrecy::{SecretString, ExposeSecret};
use std::{error::Error, str::FromStr};
use url::Url;

pub fn get_repo_list(
    url: &Url, 
    username: &Option<String>, 
    use_password: bool,
    password: &Option<SecretString>,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let base_request = client.get(url.clone());

    // Prepare the request.
    let request = if let Some(user) = username {
        if use_password && password.is_some() {
            base_request.basic_auth(user, Some(password.as_ref().unwrap().expose_secret()))
        } else {
            base_request.basic_auth(user, None::<String>)
        }
    } else {
        base_request
    };

    // Send the request and get the response.
    let response = request.send()?.text()?;

    Ok(response)
}

pub fn report_commit_log(
    url: &Url,
    username: &Option<String>,
    use_password: bool,
    password: &Option<SecretString>,
    body: String,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let base_request = client
        .request(Method::from_str("REPORT")?, url.clone())
        .header(CONTENT_TYPE, "application/xml")
        .body(body);

    let request = if let Some(user) = username {
        if use_password && password.is_some() {
            base_request.basic_auth(user, Some(password.as_ref().unwrap().expose_secret()))
        } else {
            base_request.basic_auth(user, None::<String>)
        }
    } else {
        base_request
    };

    let response = request.send()?.text()?;

    Ok(response)
}
