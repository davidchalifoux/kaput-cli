use reqwest::blocking::multipart;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Transfer {
    pub id: u32,
    pub file_id: u32,
    pub name: String,
    pub status: String,
    pub percent_done: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTransferResponse {
    pub transfers: Vec<Transfer>,
}

/// Returns the user's transfers.
pub fn list(api_token: String) -> Result<ListTransferResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response: ListTransferResponse = client
        .get("https://api.put.io/v2/transfers/list")
        .header("authorization", format!("Bearer {}", api_token))
        .send()?
        .json()?;
    Ok(response)
}

/// Starts a new transfer on the account with the given URL.
pub fn add(api_token: String, url: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new().text("url", url);
    client
        .post("https://api.put.io/v2/transfers/add")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;

    Ok(())
}

/// Cancels or removes transfers
pub fn cancel(api_token: String, transfer_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new().text("transfer_ids", transfer_id);
    client
        .post("https://api.put.io/v2/transfers/cancel")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;

    Ok(())
}

/// Clears all finished transfers
pub fn clean(api_token: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    client
        .post("https://api.put.io/v2/transfers/clean")
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;
    Ok(())
}

/// Retries failed transfers
pub fn retry(api_token: String, transfer_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new().text("id", transfer_id.to_string());
    client
        .post("https://api.put.io/v2/transfers/retry")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;

    Ok(())
}

/// Removes transfers by ID
pub fn remove(api_token: String, transfer_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new().text("transfer_ids", transfer_id);
    client
        .post("https://api.put.io/v2/transfers/remove")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;

    Ok(())
}
