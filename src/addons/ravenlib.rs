extern crate reqwest;

use std::{
    fs,
    fmt,
    error::Error,
    
    io::{
        self,
        BufRead
    }
};

use serde_json::Value;
use serde::Deserialize;
use reqwest::{Client, header};

use crate::{
    ui::ui_alerts::PaimonUIAlerts,

    configs::{
        env::Env,
        apis_uri::ApisUri,
    }, 
    
    cmd::{
        syntax::Lexico,
        download::Download,
    }
};

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug)]
enum ApiError {
    Message(String)
}

impl fmt::Display for ApiError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Message(msg) => write!(f, "{}", msg),
        }
    }

}

impl Error for ApiError {}

pub struct Ravenlib;

impl Ravenlib {
    
    fn remove_initial_character(text: &str, character: char) -> String {
        if let Some(rest) = text.strip_prefix(character) {
            return String::from(rest);
        }
        
        return String::from(text);
    }
    
    pub fn check_is_user(input: &str) -> bool {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() == 2 && input.starts_with('@') && !parts[1].is_empty() { return true }
        false
    }

    pub async fn get(list_id: &str, no_ignore: bool, no_comments: bool) -> Result<String, Box<dyn Error>> {
        let list = Self::remove_initial_character(list_id, '@');
        let mut url = ApisUri::RAVENLIB_API_REQUEST.to_owned();
    
        url.push_str(ApisUri::RAVENLIB_API_LISTS_ENDPOINT);
        url.push_str("/");
        url.push_str(&list);
        url.push_str("/raw");
    
        let client = Client::builder().danger_accept_invalid_certs(true).build()?;
        let response = client
            .get(&url)
            .header(
                header::AUTHORIZATION, format!(
                    "Bearer {}", Env::env_var("RAVENLIB_API_KEY")
                )
            )
            .send().await?;
    
        if response.status().is_success() {
            let result = String::new();
            let mut is_json = true;
            let data = response.text().await?;
    
            if let Ok(json_data) = serde_json::from_str::<Value>(&data) {
                if let Some(message) = json_data.get("message") {
                    if let Some(message_str) = message.as_str() {
                        return Ok(message_str.to_string());
                    }
                }
            } else {
                is_json = false;
            }
    
            if !is_json {
                let lines_iter = io::Cursor::new(&data).lines();
    
                for line_result in lines_iter {
                    let url = line_result?;
                    let path = Lexico::handle_get_path(&url);
                    let _ = fs::create_dir(&path);

                    Download::download_file(
                        &url, 
                        &path,
                        no_ignore, 
                        no_comments
                    ).await?;
                }
            }
    
            Ok(result)
        } else {
            let response_text = response.text().await?;
    
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                let message = ApiError::Message(error_response.message);
                PaimonUIAlerts::generic_error(&message.to_string());
    
                Ok(message.to_string())
            } else {
                Err(
                    ApiError::Message(
                        format!("Error: internal server error")
                    ).into()
                )
            }
        }
    }
    
}