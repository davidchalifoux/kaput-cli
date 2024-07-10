use std::process::{Command as ProcessCommand, Stdio};
use std::{fmt, fs};

use reqwest::blocking::multipart;
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
    pub id: u32,
    pub name: String,
    pub file_type: String,
    pub size: FileSize,
    pub created_at: String,
    #[serde_as(as = "DefaultOnNull")]
    pub parent_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilesResponse {
    pub files: Vec<File>,
    pub parent: File,
}

/// Returns the user's files.
pub fn list(
    api_token: &String,
    parent_id: u32,
) -> Result<FilesResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response: FilesResponse = client
        .get(format!(
            "https://api.put.io/v2/files/list?parent_id={parent_id}"
        ))
        .header("authorization", format!("Bearer {}", *api_token))
        .send()?
        .json()?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub files: Vec<File>,
    pub total: u32,
}

/// Searches files for given keyword.
pub fn search(
    api_token: String,
    query: String,
) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response: SearchResponse = client
        .get(format!("https://api.put.io/v2/files/search?query={query}"))
        .header("authorization", format!("Bearer {}", api_token))
        .send()?
        .json()?;
    Ok(response)
}

/// Delete file(s)
pub fn delete(api_token: String, file_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new().text("file_ids", file_id);
    client
        .post("https://api.put.io/v2/files/delete")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlResponse {
    pub url: String,
}

/// Returns a download URL for a given file.
pub fn url(api_token: &String, file_id: u32) -> Result<UrlResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response: UrlResponse = client
        .get(format!("https://api.put.io/v2/files/{file_id}/url"))
        .header("authorization", format!("Bearer {}", *api_token))
        .send()?
        .json()?;
    Ok(response)
}

/// Moves a file to a different parent
pub fn mv(
    api_token: String,
    file_id: String,
    new_parent_id: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new()
        .text("file_ids", file_id)
        .text("parent_id", new_parent_id.to_string());
    client
        .post("https://api.put.io/v2/files/move")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;

    Ok(())
}

/// Renames a file
pub fn rename(
    api_token: String,
    file_id: u32,
    new_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new()
        .text("file_id", file_id.to_string())
        .text("name", new_name);
    client
        .post("https://api.put.io/v2/files/rename")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;

    Ok(())
}

/// Extracts ZIP and RAR archives
pub fn extract(api_token: String, file_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new().text("file_ids", file_id);
    client
        .post("https://api.put.io/v2/files/extract")
        .multipart(form)
        .header("authorization", format!("Bearer {}", api_token))
        .send()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Extraction {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractionResponse {
    pub extractions: Vec<Extraction>,
}

/// Returns active extractions
pub fn get_extractions(
    api_token: String,
) -> Result<ExtractionResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response: ExtractionResponse = client
        .get("https://api.put.io/v2/files/extract")
        .header("authorization", format!("Bearer {}", api_token))
        .send()?
        .json()?;
    Ok(response)
}

// Downloads a file or folder
pub fn download(
    api_token: &String,
    file_id: u32,
    recursive: bool,
    path: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let files: FilesResponse = put::files::list(api_token, file_id).expect("querying files");

    match files.parent.file_type.as_str() {
        "FOLDER" => {
            // ID is for a folder
            match recursive {
                true => {
                    // Recursively download the folder
                    let directory_path: String = match path {
                        Some(p) => format!("{}/{}", p, files.parent.name), // Use the provided path if there is one
                        None => format!("./{}", files.parent.name),
                    };

                    fs::create_dir_all(directory_path.clone()).expect("creating directory");

                    for file in files.files {
                        download(api_token, file.id, true, Some(&directory_path))
                            .expect("downloading file recursively");
                    }
                }
                false => {
                    // Create a ZIP
                    println!("Creating ZIP...");

                    let zip_url =
                        put::zips::create(api_token, files.parent.id).expect("creating zip job");

                    println!("ZIP created!");

                    let output_path: String = match path {
                        Some(p) => format!("{}/{}.zip", p, files.parent.name),
                        None => format!("./{}.zip", files.parent.name),
                    };

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
                put::files::url(api_token, file_id).expect("creating download URL");

            let output_path: String = match path {
                Some(p) => format!("{}/{}", p, files.parent.name),
                None => format!("./{}", files.parent.name),
            };

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
