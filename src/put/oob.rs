use std::collections::HashMap;

/// Returns a new OOB code.
pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://api.put.io/v2/oauth2/oob/code?app_id=4701")?
        .json::<HashMap<String, String>>()?;
    let code = resp.get("code").expect("fetching OOB code");
    Ok(code.to_string())
}

/// Returns new OAuth token if the OOB code is linked to the user's account.
pub fn check(oob_code: String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(format!(
        "https://api.put.io/v2/oauth2/oob/code/{}",
        oob_code
    ))?
    .json::<HashMap<String, String>>()?;
    let token = resp.get("oauth_token").expect("deserializing OAuth token");
    Ok(token.to_string())
}
