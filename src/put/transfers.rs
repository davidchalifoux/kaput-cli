use reqwest::{
    blocking::{multipart::Form, Client},
    Error,
};
use serde::{Deserialize, Deserializer, Serialize};
use tabled::Tabled;

// Handles potentially null fields
// Source: https://github.com/graphql-rust/juniper/issues/735#issue-677782243
fn unwrap_or_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Transfer {
    #[serde(default, deserialize_with = "unwrap_or_default")]
    pub id: u32,
    #[serde(default, deserialize_with = "unwrap_or_default")]
    pub file_id: u32,
    #[serde(default, deserialize_with = "unwrap_or_default")]
    pub name: String,
    #[serde(default, deserialize_with = "unwrap_or_default")]
    pub status: String,
    #[serde(default, deserialize_with = "unwrap_or_default")]
    pub percent_done: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTransferResponse {
    pub transfers: Vec<Transfer>,
}

/// Returns the user's transfers.
pub fn list(client: &Client, api_token: &String) -> Result<ListTransferResponse, Error> {
    let response: ListTransferResponse = client
        .get("https://api.put.io/v2/transfers/list")
        .header("authorization", format!("Bearer {api_token}"))
        .send()?
        .json()?;

    Ok(response)
}

/// Starts a new transfer on the account with the given URL.
pub fn add(
    client: &Client,
    api_token: &String,
    url: &String,
    parent_id: Option<&u32>,
) -> Result<(), Error> {
    let parent_id: u32 = match parent_id {
        Some(id) => *id,
        None => 0,
    };

    let form: Form = Form::new()
        .text("url", url.to_owned())
        .text("save_parent_id", parent_id.to_string());

    client
        .post("https://api.put.io/v2/transfers/add")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}

/// Cancels or removes transfers
pub fn cancel(client: &Client, api_token: &String, transfer_id: u32) -> Result<(), Error> {
    let form: Form = Form::new().text("transfer_ids", transfer_id.to_string());

    client
        .post("https://api.put.io/v2/transfers/cancel")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}

/// Clears all finished transfers
pub fn clean(client: &Client, api_token: &String) -> Result<(), Error> {
    client
        .post("https://api.put.io/v2/transfers/clean")
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}

/// Retries failed transfers
pub fn retry(client: &Client, api_token: &String, transfer_id: u32) -> Result<(), Error> {
    let form: Form = Form::new().text("id", transfer_id.to_string());

    client
        .post("https://api.put.io/v2/transfers/retry")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}

/// Removes transfers by ID
pub fn remove(client: &Client, api_token: &String, transfer_id: u32) -> Result<(), Error> {
    let form: Form = Form::new().text("transfer_ids", transfer_id.to_string());

    client
        .post("https://api.put.io/v2/transfers/remove")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}
