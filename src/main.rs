use clap::{arg, Arg, Command};
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
fn require_auth(config: &ConfigFile) -> put::account::AccountResponse {
    if config.api_token.is_empty() {
        panic!("missing API key, please login to your Put.io account using the `login` command")
    }
    let account = put::account::info(config.api_token.clone())
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
                    "Logs into your Put.io by saving an auth token locally on your device.",
                ),
        )
        .subcommand(
            Command::new("logout")
                .about("Logout of your account")
                .long_about(
                "Logs out of your Put.io account by removing the auth token saved on your device.",
            ),
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
                        .arg(arg!([FOLDER_ID] "Lists the contents of a folder (optional)")),
                )
                .subcommand(
                    Command::new("search")
                        .about("Search you and your friend's files")
                        .long_about("Searches you and your friend's files.")
                        .arg_required_else_help(true)
                        .arg(arg!(<QUERY> "Keyword(s) to search for (required)")),
                )
                .subcommand(
                    Command::new("url")
                        .about("Generate a URl for downloading a file or folder")
                        .long_about("Generates a URl for downloading a file or folder.")
                        .arg_required_else_help(true)
                        .arg(arg!(<FILE_ID> "ID of a file or folder (required)")),
                )
                .subcommand(
                    Command::new("download")
                        .about("Download a file or folder")
                        .long_about("Downloads a file or folder from your account to your device.")
                        .arg_required_else_help(true)
                        .arg(arg!(<FILE_ID> "ID of a file or folder (required)")),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete file(s)")
                        .long_about("Deletes the specified file(s) on your account.")
                        .arg_required_else_help(true)
                        .arg(arg!(<FILE_ID> "ID(s) of a file (required)")),
                )
                .subcommand(
                    Command::new("upload")
                        .about("Upload file(s) to your account")
                        .long_about("Uploads file(s) to your account.")
                        .arg_required_else_help(true)
                        .arg(
                                Arg::new("parent_id")
                                .short('p')
                                .help("ID of a Put folder to upload to instead of the root folder")
                            )
                            .arg(
                                Arg::new("filename")
                                .short('n')
                                .help("Override file name")
                            )
                        .arg(
                            arg!(<PATH> ... "Valid paths of files to upload")
                                .value_parser(clap::value_parser!(PathBuf)),
                        ),
                )
                .subcommand(
                    Command::new("move")
                        .about("Move files")
                        .long_about("Moves a file to a different parent folder.")
                        .arg_required_else_help(true)
                        .arg(arg!(<FILE_ID> "ID(s) of the file(s) to move (required)"))
                        .arg(arg!(<PARENT_ID> "ID of the new parent folder (required)")),
                )
                .subcommand(
                    Command::new("rename")
                        .about("Rename files")
                        .long_about("Renames a file.")
                        .arg_required_else_help(true)
                        .arg(arg!(<FILE_ID> "ID of the file to rename (required)"))
                        .arg(arg!(<NAME> "New name for the file (required)")),
                )
                .subcommand(
                    Command::new("extractions")
                        .about("List active extractions")
                        .long_about("Lists active extractions."),
                )
                .subcommand(
                    Command::new("extract")
                        .about("Extract ZIP and RAR archives")
                        .long_about("Extracts ZIP and RAR archives.")
                        .arg_required_else_help(true)
                        .arg(arg!(<FILE_ID> "ID(s) of the file(s) to extract (required)")),
                ),
        )
        .subcommand(
            Command::new("transfers")
                .about("Manage your transfers")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List the current transfers on your account")
                        .long_about("Lists the current transfers on your account."),
                )
                .subcommand(
                    Command::new("add")
                        .about("Add new transfer with URL")
                        .long_about("Adds new transfers to your account with a URL.")
                        .arg_required_else_help(true)
                        .arg(arg!(<URL> "URL to transfer (required)")),
                )
                .subcommand(
                    Command::new("cancel")
                        .about("Cancel or remove transfers")
                        .long_about("Cancels or removes transfers on your account. If transfer is in SEEDING state, stops seeding. Otherwise, it removes the transfer entry. Does not remove their files.")
                        .arg_required_else_help(true)
                        .arg(arg!(<TRANSFER_ID> "ID of a transfer (required)")),
                )
                .subcommand(
                    Command::new("retry")
                        .about("Retry failed transfer")
                        .long_about("Retries failed transfers.")
                        .arg_required_else_help(true)
                        .arg(arg!(<TRANSFER_ID> "ID of a transfer (required)")),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove transfer(s)")
                        .long_about("Removes transfer(s).")
                        .arg_required_else_help(true)
                        .arg(arg!(<TRANSFER_ID> "ID(s) of a transfer (required)")),
                )
                .subcommand(
                    Command::new("clean")
                        .about("Clear all finshed transfers")
                        .long_about(
                            "Clears all finshed transfers on your account. Does not remove files.",
                        ),
                )
                ,
        )
        .subcommand(
            Command::new("whoami")
                .about("Check what account you are logged into")
                .long_about(
                    "Returns the username and email of the currently authenticated Put.io user.",
                ),
        )
        .subcommand(
            Command::new("debug")
                .about("Check the current config")
                .long_about("Returns the current config file and path."),
        )
}

/// Used with Confy to control the config storage location
const APP_NAME: &str = "kaput-cli";

fn main() {
    let config: ConfigFile = confy::load(APP_NAME, None).expect("reading config file");

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("login", _sub_matches)) => {
            // Create new OOB code and prompt user to link
            let oob_code = put::oob::get().expect("fetching OOB code");
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

                let get_oauth_token_result = put::oob::check(oob_code.clone());

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
            let account = require_auth(&config);
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
            Some(("list", sub_matches)) => {
                require_auth(&config);

                let folder_id_result = sub_matches.get_one::<String>("FOLDER_ID");
                let folder_id = match folder_id_result {
                    Some(folder_id) => folder_id.parse::<u32>().expect("parsing folder_id"),
                    None => 0,
                };
                let files = put::files::list(config.api_token, folder_id).expect("fetching files");
                if files.parent.file_type != "FOLDER" {
                    panic!("the ID provided should be for a folder and not a file")
                }
                let table = Table::new(&files.files).with(Style::markdown()).to_string();
                println!("\n# {}\n", &files.parent.name);
                println!("{}\n", table);
            }
            Some(("url", sub_matches)) => {
                require_auth(&config);

                let file_id = sub_matches
                    .get_one::<String>("FILE_ID")
                    .expect("missing file ID argument")
                    .parse::<u32>()
                    .expect("parsing file_id");

                let file_info = put::files::list(config.api_token.clone(), file_id)
                    .expect("fetching file info");

                if file_info.parent.file_type == "FOLDER" {
                    println!("Creating zip...");
                    let zip_url =
                        put::zips::create(config.api_token, file_id).expect("generating zip url");
                    println!("URL: {:#?}", zip_url);
                } else {
                    let download_url =
                        put::files::url(config.api_token, file_id).expect("generating url");
                    println!("URL: {:#?}", download_url.url)
                }
            }
            Some(("search", sub_matches)) => {
                require_auth(&config);

                let query = sub_matches
                    .get_one::<String>("QUERY")
                    .expect("missing query argument");
                let files = put::files::search(config.api_token, query.to_string())
                    .expect("querying files");
                let table = Table::new(&files.files).with(Style::markdown()).to_string();
                println!("\n# Results for `{}`\n", &query);
                println!("{}\n", table);
            }
            Some(("download", sub_matches)) => {
                require_auth(&config);

                let file_id = sub_matches
                    .get_one::<String>("FILE_ID")
                    .expect("missing file_id argument");
                let file_id_int = file_id.parse::<u32>().expect("parsing file_id");
                let files = put::files::list(config.api_token.clone(), file_id_int)
                    .expect("querying files");
                if files.parent.file_type != "FOLDER" {
                    // ID is for a file
                    let url_response = put::files::url(config.api_token, file_id_int)
                        .expect("creating download URL");

                    println!("Downloading: {}\n", files.parent.name);

                    // https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#redirect-both-stdout-and-stderr-of-child-process-to-the-same-file
                    ProcessCommand::new("curl")
                        .arg("-C")
                        .arg("-")
                        .arg("-o")
                        .arg(files.parent.name)
                        .arg(url_response.url)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("running CURL command")
                        .wait_with_output()
                        .expect("running CURL command");

                    println!("\nDownload finished!")
                } else {
                    // ID is for a folder
                    // Create a ZIP
                    println!("Creating ZIP...");
                    let zip_url = put::zips::create(config.api_token, files.parent.id)
                        .expect("creating zip job");
                    println!("ZIP created!");

                    println!("Downloading: {}\n", files.parent.name);

                    // https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#redirect-both-stdout-and-stderr-of-child-process-to-the-same-file
                    ProcessCommand::new("curl")
                        .arg("-C")
                        .arg("-")
                        .arg("-o")
                        .arg(files.parent.name + ".zip")
                        .arg(zip_url)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to run CURL command")
                        .wait_with_output()
                        .expect("failed to run CURL command");

                    println!("\nDownload finished!")
                }
            }
            Some(("delete", sub_matches)) => {
                require_auth(&config);

                let file_id = sub_matches
                    .get_one::<String>("FILE_ID")
                    .expect("missing file_id argument");
                put::files::delete(config.api_token, file_id.to_string()).expect("deleting file");
                println!("File deleted!");
            }
            Some(("upload", sub_matches)) => {
                require_auth(&config);

                let parent_id = sub_matches.get_one::<String>("parent_id");

                let filename = sub_matches.get_one::<String>("filename");

                let paths = sub_matches
                    .get_many::<PathBuf>("PATH")
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                for path in paths {
                    println!("Uploading: {}\n", path.to_string_lossy());
                    ProcessCommand::new("curl")
                        .arg("-H")
                        .arg(format!("Authorization: Bearer {}", config.api_token))
                        .arg("-F")
                        .arg(format!("file=@{}", path.to_string_lossy()))
                        .arg("-F")
                        .arg(format!(
                            "filename={}",
                            filename.clone().unwrap_or(&"".to_string())
                        ))
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
                require_auth(&config);

                let file_id = sub_matches
                    .get_one::<String>("FILE_ID")
                    .expect("missing file_id argument");
                let new_parent_id = sub_matches
                    .get_one::<String>("PARENT_ID")
                    .expect("missing parent_id argument")
                    .parse::<u32>()
                    .expect("parsing file_id");

                put::files::mv(config.api_token, file_id.to_string(), new_parent_id)
                    .expect("moving file(s)");
                println!("File(s) moved!");
            }
            Some(("rename", sub_matches)) => {
                require_auth(&config);

                let file_id = sub_matches
                    .get_one::<String>("FILE_ID")
                    .expect("missing file_id argument")
                    .parse::<u32>()
                    .expect("parsing file_id");
                let new_name = sub_matches
                    .get_one::<String>("NAME")
                    .expect("missing parent_id argument");

                put::files::rename(config.api_token, file_id, new_name.to_string())
                    .expect("renaming file");

                println!("File renamed!");
            }
            Some(("extractions", _sub_matches)) => {
                require_auth(&config);

                let extractions =
                    put::files::get_extractions(config.api_token).expect("fetching extractions");
                let table = Table::new(&extractions.extractions)
                    .with(Style::markdown())
                    .to_string();

                println!("\n# Active extractions\n");
                println!("{}\n", table);
            }
            Some(("extract", sub_matches)) => {
                require_auth(&config);

                let file_id = sub_matches
                    .get_one::<String>("FILE_ID")
                    .expect("missing file_id argument");

                put::files::extract(config.api_token, file_id.to_string())
                    .expect("starting extraction");

                println!("Extraction started!");
            }
            _ => {
                println!("Invalid command. Try using the `--help` flag.")
            }
        },
        Some(("transfers", sub_matches)) => match sub_matches.subcommand() {
            Some(("list", _sub_matches)) => {
                require_auth(&config);

                let transfers_response =
                    put::transfers::list(config.api_token).expect("fetching transfers");
                let table = Table::new(&transfers_response.transfers)
                    .with(Style::markdown())
                    .to_string();
                println!("\n# Your transfers\n");
                println!("{}\n", table);
            }
            Some(("add", sub_matches)) => {
                require_auth(&config);

                let url = sub_matches
                    .get_one::<String>("URL")
                    .expect("missing URL argument");
                put::transfers::add(config.api_token, url.to_string()).expect("starting transfer");
                println!("Transfer added!");
            }
            Some(("cancel", sub_matches)) => {
                require_auth(&config);

                let transfer_id = sub_matches
                    .get_one::<String>("TRANSFER_ID")
                    .expect("missing transfer_id argument");
                put::transfers::cancel(config.api_token, transfer_id.to_string())
                    .expect("cancelling transfer");
                println!("Transfer cancelled or removed!");
            }
            Some(("retry", sub_matches)) => {
                require_auth(&config);

                let transfer_id = sub_matches
                    .get_one::<String>("TRANSFER_ID")
                    .expect("missing transfer_id argument")
                    .parse::<u32>()
                    .expect("parsing file_id");

                put::transfers::retry(config.api_token, transfer_id).expect("restarting transfer");
                println!("Transfer restarted!");
            }
            Some(("remove", sub_matches)) => {
                require_auth(&config);

                let transfer_id = sub_matches
                    .get_one::<String>("TRANSFER_ID")
                    .expect("missing transfer_id argument");

                put::transfers::remove(config.api_token, transfer_id.to_string())
                    .expect("removing transfer");
                println!("Transfer removed!");
            }
            Some(("clean", _sub_matches)) => {
                require_auth(&config);

                put::transfers::clean(config.api_token).expect("clearing transfers");
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
