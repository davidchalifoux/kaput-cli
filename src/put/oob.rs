use std::collections::HashMap;

use reqwest::{blocking::Client, Error};

/// Returns a new OOB code.
pub fn get(client: &Client) -> Result<String, Error> {
    let resp = client
        .get("https://api.put.io/v2/oauth2/oob/code?app_id=4701")
        .send()?
        .json::<HashMap<String, String>>()?;

    let code: &String = resp.get("code").expect("fetching OOB code");

    Ok(code.clone())
}

/// Returns new OAuth token if the OOB code is linked to the user's account.
pub fn check(client: &Client, oob_code: &String) -> Result<String, Error> {
    let resp = client
        .get(format!("https://api.put.io/v2/oauth2/oob/code/{oob_code}"))
        .send()?
        .json::<HashMap<String, String>>()?;

    let token: &String = resp.get("oauth_token").expect("fetching OAuth token");

    Ok(token.clone())
}
