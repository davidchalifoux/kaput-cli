use clap::{value_parser, Arg, Command};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Stdio};
use std::{thread, time};
use tabled::{settings::Style, Table};

mod put;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    api_token: String,
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            api_token: "".into(),
        }
    }
}

/// Verifies that the user has a valid API key set and that their account is still active
fn require_auth(client: &Client, config: &ConfigFile) -> put::account::AccountResponse {
    if config.api_token.is_empty() {
        panic!("missing API key, please login to your Put.io account using the `login` command")
    }
    let account = put::account::info(client, &config.api_token)
        .expect("invalid OAuth token in config, login again using the `login` command");
    if !account.info.account_active {
        panic!("inactive Put.io account")
    }

    account
}

fn cli() -> Command {
    Command::new("kaput")
        .about("The unofficial CLI for Put.io")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("login")
                .about("Login to your Put.io account")
                .long_about(
                    "Logs into your Put.io by saving an auth token locally on your device."
                )
        )
        .subcommand(
            Command::new("logout")
                .about("Logout of your account")
                .long_about(
                    "Logs out of your Put.io account by removing the auth token saved on your device."
                )
        )
        .subcommand(
            Command::new("files")
                .about("Manage your files")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List your files and folders")
                        .long_about("Lists your files and folders.")
                        .arg(
                            Arg::new("FOLDER_ID")
                            .help("Lists the contents of a folder (optional)")
                            .value_parser(value_parser!(i64))
                            .required(false)
                            .num_args(1)
                        )
                        .arg(
                            Arg::new("self")
                            .short('s')
                            .long("self")
                            .help("If set, returns the info for the folder itself in JSON format")
                            .required(false)
                            .num_args(0)
                        )
                        .arg(
                            Arg::new("json")
                            .long("json")
                            .help("If set, returns the output in JSON format")
                            .required(false)
                            .num_args(0)
                        )
                )
                .subcommand(
                    Command::new("search")
                        .about("Search you and your friend's files")
                        .long_about("Searches you and your friend's files.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("QUERY")
                            .required(true)
                            .help("Keyword(s) to search for (required)")
                        )
                )
                .subcommand(
                    Command::new("url")
                        .about("Generate a URl for downloading a file or folder")
                        .long_about("Generates a URl for downloading a file or folder.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("FILE_ID")
                            .value_parser(value_parser!(i64))
                            .required(true)
                            .help("ID of a file or folder (required)")
                        )
                )
                .subcommand(
                    Command::new("download")
                        .about("Download a file or folder")
                        .long_about("Downloads a file or folder from your account to your device.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("FILE_ID")
                            .value_parser(value_parser!(i64))
                            .required(true)
                            .help("ID(s) of a file or folder (required)")
                        )
                        .arg(
                            Arg::new("path")
                            .short('p')
                            .long("path")
                            .help("Path to download the file(s) to")
                            .required(false).num_args(1)
                        )
                        .arg(
                            Arg::new("recursive")
                            .short('r')
                            .long("recursive")
                            .help("Download the contents of a folder recursively without creating a zip")
                            .required(false)
                            .num_args(0)
                        )
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete file(s)")
                        .long_about("Deletes the specified file(s) on your account.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("FILE_ID")
                            .value_parser(value_parser!(i64))
                            .required(true)
                            .help("ID(s) of a file (required)")
                        )
                )
                .subcommand(
                    Command::new("upload")
                        .about("Upload file(s) to your account")
                        .long_about("Uploads file(s) to your account.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("parent_id")
                                .short('p')
                                .long("parent")
                                .value_parser(value_parser!(i64))
                                .help("ID of a Put folder to upload to instead of the root folder")
                                .required(false)
                        )
                        .arg(
                            Arg::new("file_name")
                                .short('n')
                                .long("name")
                                .help("Override file name")
                                .required(false)
                        )
                        .arg(
                            Arg::new("is_silent")
                                .short('s')
                                .long("silent")
                                .help("Run CURL in silent mode")
                                .required(false)
                                .num_args(0)
                        )
                        .arg(
                            Arg::new("PATH")
                                .required(true)
                                .help("Valid paths of files to upload")
                                .value_parser(value_parser!(PathBuf))
                        )
                )
                .subcommand(
                    Command::new("move")
                        .about("Move files")
                        .long_about("Moves a file to a different parent folder.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("FILE_ID")
                            .help("ID of the file to move (required)")
                            .value_parser(value_parser!(i64))
                            .required(true))
                        .arg(
                            Arg::new("PARENT_ID")
                            .help("ID of the new parent folder (required)")
                            .value_parser(value_parser!(i64))
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("rename")
                        .about("Rename files")
                        .long_about("Renames a file.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("FILE_ID")
                            .help("ID of the file to rename (required)")
                            .value_parser(value_parser!(i64))
                            .required(true)
                        )
                        .arg(
                            Arg::new("NAME")
                            .help("New name for the file (required)")
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("extractions")
                        .about("List active extractions")
                        .long_about("Lists active extractions.")
                )
                .subcommand(
                    Command::new("extract")
                        .about("Extract ZIP and RAR archives")
                        .long_about("Extracts ZIP and RAR archives.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("FILE_ID")
                            .help("ID of the file to extract (required)")
                            .value_parser(value_parser!(i64))
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("play")
                        .about("Stream a video file")
                        .long_about(
                            "Plays a video file using MPV.\n\
                            If you do not have MPV installed, visit https://mpv.io/installation/.",
                        )
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("FILE_ID")
                            .help("ID of a video file (required)")
                            .value_parser(value_parser!(i64))
                            .required(true)
                        )
                )
        )
        .subcommand(
            Command::new("transfers")
                .about("Manage your transfers")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List the current transfers on your account")
                        .long_about("Lists the current transfers on your account.")
                )
                .subcommand(
                    Command::new("add")
                        .about("Add new transfer with URL")
                        .long_about("Adds new transfers to your account with a URL.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("URL")
                            .help("URL to transfer (required)")
                            .required(true)
                        )
                        .arg(
                            Arg::new("parent_id")
                            .short('p')
                            .long("parent")
                            .value_parser(value_parser!(i64))
                            .help("ID of a Put folder to upload to instead of the root folder")
                            .required(false)
                        )
                )
                .subcommand(
                    Command::new("cancel")
                        .about("Cancel or remove transfers")
                        .long_about("Cancels or removes transfers on your account. If transfer is in SEEDING state, stops seeding. Otherwise, it removes the transfer entry. Does not remove their files.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("TRANSFER_ID")
                            .help("ID of a transfer (required)")
                            .value_parser(value_parser!(i64))
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("retry")
                        .about("Retry failed transfer")
                        .long_about("Retries failed transfers.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("TRANSFER_ID")
                            .help("ID of a transfer (required)")
                            .value_parser(value_parser!(i64))
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove transfer(s)")
                        .long_about("Removes transfer(s).")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("TRANSFER_ID")
                            .help("ID of a transfer (required)")
                            .value_parser(value_parser!(i64))
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("clean")
                        .about("Clear all finished transfers")
                        .long_about(
                            "Clears all finished transfers on your account. Does not remove files."
                        )
                )
        )
        .subcommand(
            Command::new("whoami")
                .about("Check what account you are logged into")
                .long_about(
                    "Returns the username and email of the currently authenticated Put.io user."
                )
        )
        .subcommand(
            Command::new("debug")
                .about("Check the current config")
                .long_about("Returns the current config file and path.")
        )
}

/// Used with Confy to control the config storage location
const APP_NAME: &str = "kaput-cli";

fn main() {
    let config: ConfigFile = confy::load(APP_NAME, None).expect("reading config file");

    let matches: clap::ArgMatches = cli().get_matches();

    let client: Client = reqwest::blocking::Client::new();

    match matches.subcommand() {
        Some(("login", _sub_matches)) => {
            // Create new OOB code and prompt user to link
            let oob_code = put::oob::get(&client).expect("fetching OOB code");

            println!(
                "Go to https://put.io/link and enter the code: {:#?}",
                oob_code
            );
            println!("Waiting for link...");

            // Every three seconds, check if the OOB code was linked to the user's account
            // If linked, update the config file
            // Stops after 10 tries (30 seconds)
            let three_seconds = time::Duration::from_secs(3);
            let mut try_count = 0;
            loop {
                try_count += 1;

                thread::sleep(three_seconds);

                let get_oauth_token_result = put::oob::check(&client, &oob_code);

                let oauth_token = match get_oauth_token_result {
                    Ok(token) => token,
                    Err(_error) => {
                        continue;
                    }
                };

                if !oauth_token.is_empty() {
                    let cfg = ConfigFile {
                        api_token: oauth_token,
                    };
                    confy::store(APP_NAME, None, cfg).expect("updating OAuth token");
                    println!("Signed-in successfully!");
                    break;
                }
                if try_count > 10 {
                    panic!("Took too long to verify code. Try again and make sure to link to your account within 30 seconds.");
                }
            }
        }
        Some(("logout", _sub_matches)) => {
            let cfg = ConfigFile {
                api_token: "".into(),
            };
            confy::store(APP_NAME, None, cfg).expect("updating config file");
            println!("Signed out successfully!")
        }
        Some(("whoami", _sub_matches)) => {
            let account: put::account::AccountResponse = require_auth(&client, &config);

            println!(
                "Logged in as {} ({})",
                account.info.username, account.info.mail
            )
        }
        Some(("debug", _sub_matches)) => {
            let config_path = confy::get_configuration_file_path(APP_NAME, None)
                .expect("getting config file path");
            println!("Config path: {:#?}", config_path);
            println!("Config:");
            println!("{:#?}", config);
        }
        Some(("files", sub_matches)) => match sub_matches.subcommand() {
            Some(("play", sub_matches)) => {
                require_auth(&client, &config);

                let file_id = sub_matches.get_one("FILE_ID").expect("missing file ID");

                let file_info: put::files::FilesResponse =
                    put::files::list(&client, &config.api_token, *file_id)
                        .expect("fetching file info");

                if file_info.parent.file_type != "VIDEO" {
                    println!("File type must be video.");
                    return;
                }

                let download_url: put::files::UrlResponse =
                    put::files::url(&client, &config.api_token, *file_id).expect("generating url");

                ProcessCommand::new("mpv")
                    .arg(download_url.url)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("error while spawning mpv (is it installed?)")
                    .wait()
                    .expect("error while running MPV");
            }
            Some(("list", sub_matches)) => {
                require_auth(&client, &config);

                let folder_id_result = sub_matches.get_one("FOLDER_ID");

                let folder_id: i64 = match folder_id_result {
                    Some(folder_id) => *folder_id,
                    None => 0,
                };

                let files = put::files::list(&client, &config.api_token, folder_id)
                    .expect("fetching files");

                let should_only_show_self: Option<&bool> = sub_matches.get_one::<bool>("self");

                if *should_only_show_self.unwrap_or(&false) {
                    // Only show info for the parent
                    println!("{}", serde_json::to_string_pretty(&files.parent).unwrap());
                    return;
                }

                if files.parent.file_type != "FOLDER" {
                    println!("The ID provided should be for a folder and not a file");
                    return;
                }

                let should_return_json = sub_matches.get_one::<bool>("json");
                if *should_return_json.unwrap_or(&false) {
                    // Return in JSON format
                    println!("{}", serde_json::to_string_pretty(&files.files).unwrap());
                    return;
                }

                // Return table format
                let table = Table::new(&files.files).with(Style::markdown()).to_string();
                println!("\n# {}\n", &files.parent.name);
                println!("{}\n", table);
            }
            Some(("url", sub_matches)) => {
                require_auth(&client, &config);

                let file_id: &i64 = sub_matches
                    .get_one("FILE_ID")
                    .expect("missing file ID argument");

                let file_info = put::files::list(&client, &config.api_token, *file_id)
                    .expect("fetching file info");

                if file_info.parent.file_type == "FOLDER" {
                    println!("Creating zip...");

                    let zip_url = put::zips::create(&client, &config.api_token, *file_id)
                        .expect("generating zip url");

                    println!("URL: {:#?}", zip_url);
                } else {
                    let download_url = put::files::url(&client, &config.api_token, *file_id)
                        .expect("generating url");

                    println!("URL: {:#?}", download_url.url)
                }
            }
            Some(("search", sub_matches)) => {
                require_auth(&client, &config);

                let query = sub_matches
                    .get_one::<String>("QUERY")
                    .expect("missing query argument");

                let files =
                    put::files::search(&client, &config.api_token, query).expect("querying files");

                let table = Table::new(files.files).with(Style::markdown()).to_string();

                println!("\n# Results for `{}`\n", &query);
                println!("{}\n", table);
            }
            Some(("download", sub_matches)) => {
                require_auth(&client, &config);

                let recursive = sub_matches.get_flag("recursive");

                let path = sub_matches.get_one::<String>("path");

                let file_id = sub_matches
                    .get_one("FILE_ID")
                    .expect("missing file_id argument");

                put::files::download(&client, &config.api_token, *file_id, recursive, path)
                    .expect("downloading file(s)");
            }
            Some(("delete", sub_matches)) => {
                require_auth(&client, &config);

                let file_id = sub_matches
                    .get_one::<String>("FILE_ID")
                    .expect("missing file_id argument");

                put::files::delete(&client, &config.api_token, file_id).expect("deleting file");

                println!("File deleted!");
            }
            Some(("upload", sub_matches)) => {
                require_auth(&client, &config);

                let parent_id = sub_matches.get_one::<String>("parent_id");

                let file_name = sub_matches.get_one::<String>("file_name");

                let is_silent = sub_matches.get_one::<bool>("is_silent");

                let mut curl_args: Vec<String> = vec![];

                if *is_silent.unwrap_or(&false) {
                    // Run CURL in silent mode
                    curl_args.push("-s".to_string());
                }

                let paths = sub_matches
                    .get_many::<PathBuf>("PATH")
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();

                for path in paths {
                    println!("Uploading: {}\n", path.to_string_lossy());

                    ProcessCommand::new("curl")
                        .args(curl_args.clone())
                        .arg("-H")
                        .arg(format!("Authorization: Bearer {}", config.api_token))
                        .arg("-F")
                        .arg(format!("file=@{}", path.to_string_lossy()))
                        .arg("-F")
                        .arg(format!("filename={}", file_name.unwrap_or(&"".to_string())))
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
            }
            Some(("move", sub_matches)) => {
                require_auth(&client, &config);

                let file_id = sub_matches
                    .get_one("FILE_ID")
                    .expect("missing file_id argument");

                let new_parent_id = sub_matches
                    .get_one("PARENT_ID")
                    .expect("missing parent_id argument");

                put::files::mv(&client, &config.api_token, *file_id, *new_parent_id)
                    .expect("moving file(s)");

                println!("File(s) moved!");
            }
            Some(("rename", sub_matches)) => {
                require_auth(&client, &config);

                let file_id = sub_matches
                    .get_one("FILE_ID")
                    .expect("missing file_id argument");

                let new_name = sub_matches
                    .get_one("NAME")
                    .expect("missing parent_id argument");

                put::files::rename(&client, &config.api_token, *file_id, new_name)
                    .expect("renaming file");

                println!("File renamed!");
            }
            Some(("extractions", _sub_matches)) => {
                require_auth(&client, &config);

                let extractions = put::files::get_extractions(&client, &config.api_token)
                    .expect("fetching extractions");

                let table = Table::new(extractions.extractions)
                    .with(Style::markdown())
                    .to_string();

                println!("\n# Active extractions\n");
                println!("{}\n", table);
            }
            Some(("extract", sub_matches)) => {
                require_auth(&client, &config);

                let file_id = sub_matches
                    .get_one("FILE_ID")
                    .expect("missing file_id argument");

                put::files::extract(&client, &config.api_token, *file_id)
                    .expect("starting extraction");

                println!("Extraction started!");
            }
            _ => {
                println!("Invalid command. Try using the `--help` flag.")
            }
        },
        Some(("transfers", sub_matches)) => match sub_matches.subcommand() {
            Some(("list", _sub_matches)) => {
                require_auth(&client, &config);

                let transfers_response =
                    put::transfers::list(&client, &config.api_token).expect("fetching transfers");

                let table = Table::new(transfers_response.transfers)
                    .with(Style::markdown())
                    .to_string();

                println!("\n# Your transfers\n");
                println!("{}\n", table);
            }
            Some(("add", sub_matches)) => {
                require_auth(&client, &config);

                let url = sub_matches.get_one("URL").expect("missing URL argument");

                let parent = sub_matches.get_one("parent_id");

                put::transfers::add(&client, &config.api_token, url, parent)
                    .expect("starting transfer");

                println!("Transfer added!");
            }
            Some(("cancel", sub_matches)) => {
                require_auth(&client, &config);

                let transfer_id = sub_matches
                    .get_one("TRANSFER_ID")
                    .expect("missing transfer_id argument");

                put::transfers::cancel(&client, &config.api_token, *transfer_id)
                    .expect("cancelling transfer");

                println!("Transfer cancelled or removed!");
            }
            Some(("retry", sub_matches)) => {
                require_auth(&client, &config);

                let transfer_id = sub_matches
                    .get_one("TRANSFER_ID")
                    .expect("missing transfer_id argument");

                put::transfers::retry(&client, &config.api_token, *transfer_id)
                    .expect("retrying transfer");

                println!("Transfer restarted!");
            }
            Some(("remove", sub_matches)) => {
                require_auth(&client, &config);

                let transfer_id = sub_matches
                    .get_one("TRANSFER_ID")
                    .expect("missing transfer_id argument");

                put::transfers::remove(&client, &config.api_token, *transfer_id)
                    .expect("removing transfer");

                println!("Transfer removed!");
            }
            Some(("clean", _sub_matches)) => {
                require_auth(&client, &config);

                put::transfers::clean(&client, &config.api_token).expect("clearing transfers");

                println!("Transfers cleaned!");
            }
            _ => {
                println!("Invalid command. Try using the `--help` flag.")
            }
        },

        _ => {
            println!("Invalid command. Try using the `--help` flag.")
        }
    }
}
