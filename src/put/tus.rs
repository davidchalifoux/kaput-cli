use blake2::{Blake2b512, Digest};
use std::{
    io::{BufReader, Read},
    path::PathBuf,
    time::Instant,
};

use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking::Client;

pub fn upload(client: &Client, api_token: &String, path: &PathBuf, parent_id: Option<&String>) {
    if !path.is_file() {
        println!("{} is not a file", path.to_string_lossy());
        return;
    }

    let file_name: String = path.file_name().unwrap().to_string_lossy().to_string();

    let absolute_path: String = path
        .canonicalize()
        .expect("canonicalizing path")
        .to_string_lossy()
        .to_string();

    let metadata: std::fs::Metadata = std::fs::metadata(path).expect("reading file metadata");

    let file_size: u64 = metadata.len();
    let last_modified: std::time::SystemTime = metadata.modified().expect("reading last modified");
    let last_modified_unix: u64 = last_modified
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    println!("Uploading: {}", file_name);

    let file: std::fs::File = std::fs::File::open(path).expect("opening file");

    let location: String;

    // Check if previous location exists
    let temp_dir: PathBuf = std::env::temp_dir();
    let mut hasher = Blake2b512::new();
    hasher.update(format!("{}_{}", absolute_path, last_modified_unix));
    let hash: String = format!("{:x}", hasher.finalize());
    let temp_file_path: PathBuf = temp_dir.join(format!("kaput_{hash}"));

    if temp_file_path.exists() {
        // Read the location from the file
        println!("Resuming upload...");

        location = std::fs::read_to_string(&temp_file_path).expect("reading temp file");
    } else {
        // Get a new upload location and write it to the temp directory
        location = create_upload(client, api_token, file_size, file_name.clone(), parent_id)
            .expect("creating upload location");

        std::fs::write(&temp_file_path, location.clone()).expect("writing temp file");
    }

    let mut reader: BufReader<std::fs::File> = BufReader::new(file);

    let mut total_bytes_read: u64 = 0;

    // When resuming an upload, the server will respond with the offset at which the upload should start
    let resume_offset: u64 = get_offset(client, api_token, location.clone());

    let mut chunk: Vec<u8> = vec![0; 52_428_800]; // 50 MB chunk size

    loop {
        let bytes_read: usize = reader.read(&mut chunk).expect("Reading chunk from file");

        if bytes_read == 0 {
            // The entire file has been read

            // Delete the temp file
            std::fs::remove_file(&temp_file_path).expect("deleting temp file");

            println!("Upload finished!");
            break;
        }

        // The start and end offsets of the current chunk
        let chunk_start_offset: u64 = total_bytes_read;
        let chunk_end_offset: u64 = total_bytes_read + bytes_read as u64;

        // The total number of bytes read from the file
        total_bytes_read += bytes_read as u64;

        // Skip to the initial offset, then start uploading the file from the containing chunk
        if total_bytes_read < resume_offset {
            continue;
        }

        // The offset at which the current chunk should start uploading
        let mut chunk_skip_offset: usize = 0;

        // The file offset to send to the server
        let mut upload_offset: u64 = chunk_start_offset;

        if resume_offset >= chunk_start_offset && resume_offset < chunk_end_offset {
            // The initial offset is within the current chunk
            chunk_skip_offset = resume_offset as usize - chunk_start_offset as usize;
            upload_offset = resume_offset;
        }

        // Measure the time taken to upload the chunk
        let start_time: Instant = Instant::now();

        // Upload the chunk
        let res = client
            .patch(location.clone())
            .header("authorization", format!("Bearer {api_token}"))
            .header("tus-resumable", "1.0.0")
            .header("upload-offset", format!("{upload_offset}"))
            .header("content-type", "application/offset+octet-stream")
            .header("content-length", format!("{bytes_read}"))
            .body(chunk[chunk_skip_offset..bytes_read].to_vec())
            .send();

        let elapsed_time: f64 = start_time.elapsed().as_secs_f64();
        let upload_speed: f64 = bytes_read as f64 / elapsed_time / 1_048_576.0; // Speed in MB/s

        if let Ok(response) = res {
            if response.status() != 204 {
                println!("Error: {}", response.status());
                panic!("Upload failed, try again in a few seconds.");
            }
        } else {
            println!("Error: {:?}", res);
            panic!("Upload failed, try again in a few seconds.");
        }

        let percentage_completed: f64 = (total_bytes_read as f64 / file_size as f64) * 100.0;
        println!("{:.0}% ({:.2} MB/s)", percentage_completed, upload_speed);
    }
}

fn get_offset(client: &Client, api_token: &String, location: String) -> u64 {
    let res = client
        .head(location)
        .header("authorization", format!("Bearer {api_token}"))
        .header("tus-resumable", "1.0.0")
        .send();

    if let Ok(response) = res {
        if response.status() == 200 {
            if let Some(offset) = response.headers().get("upload-offset") {
                return offset
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .expect("Invalid offset value");
            }
        }
    }

    0
}

/// Creates a new upload using TUS, returns the location
fn create_upload(
    client: &Client,
    api_token: &String,
    file_size: u64,
    file_name: String,
    parent_id: Option<&String>,
) -> Option<String> {
    let base64_name: String = general_purpose::STANDARD.encode(file_name);
    let base64_parent_id: String =
        general_purpose::STANDARD.encode(parent_id.unwrap_or(&"0".to_string()));
    let base64_no_torrent: String = general_purpose::STANDARD.encode("true");

    let res = client
        .post("https://upload.put.io/files/")
        .header("authorization", format!("Bearer {api_token}"))
        .header("tus-resumable", "1.0.0")
        .header("upload-length", format!("{file_size}"))
        .header(
            "upload-metadata",
            format!(
                "name {base64_name},no-torrent {base64_no_torrent},parent_id {base64_parent_id}"
            ),
        )
        .send();

    if let Ok(response) = res {
        if response.status() == 201 {
            if let Some(location) = response.headers().get("location") {
                return Some(location.to_str().unwrap().to_string());
            }
        }
    }

    None
}
