use std::{thread, time};

use reqwest::blocking::multipart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateZipResponse {
    pub zip_id: u32,
}

/// Creates a new ZIP job with the given file id.
///
/// Waits for the zip job complete, and returns a string with the download URL.
pub fn create(api_token: String, file_id: u32) -> Result<String, Box<dyn std::error::Error>> {
    // Start ZIP job
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new().text("file_ids", format!("{file_id}"));
    let response: CreateZipResponse = client
        .post("https://api.put.io/v2/zips/create")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?
        .json()?;

    // Wait for ZIP job to finish
    loop {
        let check_zip_response =
            get(api_token.clone(), response.zip_id).expect("checking zip status");

        if let Some(zip_url) = check_zip_response.url {
            return Ok(zip_url);
        }

        thread::sleep(time::Duration::from_secs(3));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckZipResponse {
    pub zip_status: String,
    pub url: Option<String>,
}

/// Checks the status of a given zip job
pub fn get(api_token: String, zip_id: u32) -> Result<CheckZipResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let response: CheckZipResponse = client
        .get(format!("https://api.put.io/v2/zips/{zip_id}"))
        .header("authorization", format!("Bearer {}", api_token))
        .send()?
        .json()?;
    Ok(response)
}
