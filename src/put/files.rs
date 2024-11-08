use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Stdio};
use std::{fmt, fs};

use reqwest::blocking::multipart::Form;
use reqwest::blocking::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};
use tabled::Tabled;

use crate::put;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSize(u64);

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bytefmt::format(self.0))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub file_type: String,
    pub size: FileSize,
    pub created_at: String,
    #[serde_as(as = "DefaultOnNull")]
    pub parent_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilesResponse {
    pub files: Vec<File>,
    pub parent: File,
}

/// Returns the user's files.
pub fn list(client: &Client, api_token: &String, parent_id: i64) -> Result<FilesResponse, Error> {
    let response: FilesResponse = client
        .get(format!(
            "https://api.put.io/v2/files/list?parent_id={parent_id}"
        ))
        .header("authorization", format!("Bearer {api_token}"))
        .send()?
        .json()?;

    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub files: Vec<File>,
    pub total: i64,
}

/// Searches files for given keyword.
pub fn search(
    client: &Client,
    api_token: &String,
    query: &String,
) -> Result<SearchResponse, Error> {
    let response: SearchResponse = client
        .get(format!("https://api.put.io/v2/files/search?query={query}"))
        .header("authorization", format!("Bearer {api_token}"))
        .send()?
        .json()?;

    Ok(response)
}

/// Delete file(s)
pub fn delete(client: &Client, api_token: &String, file_id: &str) -> Result<(), Error> {
    let form: Form = Form::new().text("file_ids", file_id.to_owned());

    client
        .post("https://api.put.io/v2/files/delete")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlResponse {
    pub url: String,
}

/// Returns a download URL for a given file.
pub fn url(client: &Client, api_token: &String, file_id: i64) -> Result<UrlResponse, Error> {
    let response: UrlResponse = client
        .get(format!("https://api.put.io/v2/files/{file_id}/url"))
        .header("authorization", format!("Bearer {api_token}"))
        .send()?
        .json()?;

    Ok(response)
}

/// Moves a file to a different parent
pub fn mv(
    client: &Client,
    api_token: &String,
    file_id: i64,
    new_parent_id: i64,
) -> Result<(), Error> {
    let form: Form = Form::new()
        .text("file_ids", file_id.to_string())
        .text("parent_id", new_parent_id.to_string());

    client
        .post("https://api.put.io/v2/files/move")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}

/// Renames a file
pub fn rename(
    client: &Client,
    api_token: &String,
    file_id: i64,
    new_name: &String,
) -> Result<(), Error> {
    let form = Form::new()
        .text("file_id", file_id.to_string())
        .text("name", new_name.to_owned());

    client
        .post("https://api.put.io/v2/files/rename")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}

/// Extracts ZIP and RAR archives
pub fn extract(client: &Client, api_token: &String, file_id: i64) -> Result<(), Error> {
    let form: Form = Form::new().text("file_ids", file_id.to_string());

    client
        .post("https://api.put.io/v2/files/extract")
        .multipart(form)
        .header("authorization", format!("Bearer {api_token}"))
        .send()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Extraction {
    pub id: String,
    pub name: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractionResponse {
    pub extractions: Vec<Extraction>,
}

/// Returns active extractions
pub fn get_extractions(client: &Client, api_token: &String) -> Result<ExtractionResponse, Error> {
    let response: ExtractionResponse = client
        .get("https://api.put.io/v2/files/extract")
        .header("authorization", format!("Bearer {api_token}"))
        .send()?
        .json()?;

    Ok(response)
}

struct ReplaceChar<'a> {
    from: &'a str,
    to: &'a str,
}

/// Replaces illegal characters in a file name
fn replace_illegal_chars(name: &str) -> String {
    let mut name: String = name.to_owned();

    const ILLEGAL_CHARS: [ReplaceChar<'_>; 7] = [
        ReplaceChar { from: "<", to: "" },
        ReplaceChar { from: ">", to: "" },
        ReplaceChar {
            from: ":",
            to: " - ",
        },
        ReplaceChar {
            from: "\"",
            to: "\'",
        },
        ReplaceChar { from: "|", to: "" },
        ReplaceChar { from: "?", to: "" },
        ReplaceChar { from: "*", to: "" },
    ];

    for replacement in ILLEGAL_CHARS {
        name = name.replace(replacement.from, replacement.to);
    }

    name
}

/// Downloads a file or folder
///
/// # Arguments
///
/// * `client` - The reqwest client
/// * `api_token` - The user's API token
/// * `file_id` - The ID of the file or folder to download
/// * `recursive` - Recursively download the folder
/// * `path` - The path to save the file or folder to
/// * `no_replace` - Do not replace illegal characters in the file name
pub fn download(
    client: &Client,
    api_token: &String,
    file_id: i64,
    recursive: bool,
    path: Option<&String>,
    no_replace: bool,
) -> Result<(), Error> {
    let files: FilesResponse =
        put::files::list(client, api_token, file_id).expect("querying files");

    match files.parent.file_type.as_str() {
        "FOLDER" => {
            // ID is for a folder
            match recursive {
                true => {
                    // Recursively download the folder
                    let mut directory_path: String = match path {
                        Some(p) => format!("{}/{}", p, files.parent.name), // Use the provided path if there is one
                        None => format!("./{}", files.parent.name),
                    };

                    if !no_replace {
                        directory_path = replace_illegal_chars(&directory_path);
                    }

                    fs::create_dir_all(directory_path.clone()).expect("creating directory");

                    for file in files.files {
                        download(
                            client,
                            api_token,
                            file.id,
                            true,
                            Some(&directory_path),
                            no_replace,
                        )
                        .expect("downloading file recursively");
                    }
                }
                false => {
                    // Create a ZIP
                    println!("Creating ZIP...");

                    let zip_url: String = put::zips::create(client, api_token, files.parent.id)
                        .expect("creating zip job");

                    println!("ZIP created!");

                    let mut output_path: String = match path {
                        Some(p) => format!("{}/{}.zip", p, files.parent.name),
                        None => format!("./{}.zip", files.parent.name),
                    };

                    if !no_replace {
                        output_path = replace_illegal_chars(&output_path);
                    }

                    println!("Downloading: {}", files.parent.name);
                    println!("Saving to: {}\n", output_path);

                    // https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#redirect-both-stdout-and-stderr-of-child-process-to-the-same-file
                    ProcessCommand::new("curl")
                        .arg("-C")
                        .arg("-")
                        .arg("-o")
                        .arg(output_path)
                        .arg(zip_url)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to run CURL command")
                        .wait_with_output()
                        .expect("failed to run CURL command");

                    println!("\nDownload finished!\n")
                }
            }
        }
        _ => {
            // ID is for a file
            let url_response: UrlResponse =
                put::files::url(client, api_token, file_id).expect("creating download URL");

            let mut output_path: String = match path {
                Some(p) => format!("{}/{}", p, files.parent.name),
                None => format!("./{}", files.parent.name),
            };

            if !no_replace {
                output_path = replace_illegal_chars(&output_path);
            }

            println!("Downloading: {}", files.parent.name);
            println!("Saving to: {}\n", output_path);

            // https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#redirect-both-stdout-and-stderr-of-child-process-to-the-same-file
            ProcessCommand::new("curl")
                .arg("-C")
                .arg("-")
                .arg("-o")
                .arg(output_path)
                .arg(url_response.url)
                .stdout(Stdio::piped())
                .spawn()
                .expect("error while spawning curl")
                .wait_with_output()
                .expect("running CURL command");

            println!("\nDownload finished!\n")
        }
    }

    Ok(())
}

pub fn upload(
    api_token: &String,
    path: &PathBuf,
    parent_id: Option<&String>,
    curl_args: &Vec<String>,
) {
    println!("Uploading: {}\n", path.to_string_lossy());

    ProcessCommand::new("curl")
        .args(curl_args.clone())
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", api_token))
        .arg("-F")
        .arg(format!("file=@{}", path.to_string_lossy()))
        .arg("-F")
        .arg(format!(
            "parent_id={}",
            parent_id.unwrap_or(&"0".to_string())
        ))
        .arg("https://upload.put.io/v2/files/upload")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run CURL command")
        .wait_with_output()
        .expect("failed to run CURL command");
    println!("\nUpload finished!")
}
