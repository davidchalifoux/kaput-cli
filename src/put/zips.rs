use std::{thread, time};

use reqwest::{
    blocking::{multipart::Form, Client},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateZipResponse {
    pub zip_id: i64,
}

/// Creates a new ZIP job with the given file id.
///
/// Waits for the zip job complete, and returns a string with the download URL.
pub fn create(client: &Client, api_token: &String, file_id: i64) -> Result<String, Error> {
    // Start ZIP job
    let form: Form = Form::new().text("file_ids", file_id.to_string());

    let response: CreateZipResponse = client
        .post("https://api.put.io/v2/zips/create")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?
        .json()?;

    // Wait for ZIP job to finish
    loop {
        let check_zip_response =
            get(client, api_token, response.zip_id).expect("checking zip status");

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
pub fn get(client: &Client, api_token: &String, zip_id: i64) -> Result<CheckZipResponse, Error> {
    let response: CheckZipResponse = client
        .get(format!("https://api.put.io/v2/zips/{zip_id}"))
        .header("authorization", format!("Bearer {api_token}"))
        .send()?
        .json()?;

    Ok(response)
}
