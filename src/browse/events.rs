use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use reqwest::blocking::Client;

use super::app::{file_actions_for, AppState, BrowserApp, ModalState, PendingAction};
use crate::put;

pub fn handle_key(app: &mut BrowserApp, key: KeyEvent, client: &Client, api_token: &String) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app.app_state = AppState::Quitting;
        return;
    }

    match &app.modal {
        ModalState::Loading => {}

        ModalState::Error(_) | ModalState::Success(_) => {
            app.modal = ModalState::None;
        }

        ModalState::ConfirmDelete { file_id, .. } => {
            let file_id = *file_id;
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    app.save_position_for_reload();
                    app.pending_action = PendingAction::Delete { file_id };
                    app.spinner_label = "Deleting...".to_string();
                    app.modal = ModalState::Loading;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    app.modal = ModalState::None;
                }
                _ => {}
            }
        }

        ModalState::FileActions {
            file_id,
            file_name,
            file_type,
            selected,
        } => {
            // Extract owned copies so the borrow on app.modal ends.
            let file_id = *file_id;
            let selected = *selected;
            let file_name = file_name.clone();
            let file_type = file_type.clone();
            let in_search = app.is_search_results;
            let actions = file_actions_for(&file_type, in_search);
            let n = actions.len();

            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    let new = if selected == 0 { n - 1 } else { selected - 1 };
                    app.modal = ModalState::FileActions {
                        file_id,
                        file_name,
                        file_type,
                        selected: new,
                    };
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.modal = ModalState::FileActions {
                        file_id,
                        file_name,
                        file_type: file_type.clone(),
                        selected: (selected + 1) % n,
                    };
                }
                KeyCode::Enter => {
                    let label = actions[selected].label;
                    app.modal = ModalState::None;
                    execute_file_action(app, label, file_id, &file_type, api_token, client);
                }
                KeyCode::Char(c) => {
                    if let Some(action) = actions.iter().find(|a| a.key == c) {
                        let label = action.label;
                        app.modal = ModalState::None;
                        execute_file_action(app, label, file_id, &file_type, api_token, client);
                    }
                }
                KeyCode::Esc => {
                    app.modal = ModalState::None;
                }
                _ => {}
            }
        }

        ModalState::SearchInput { query } => {
            let query = query.clone();
            match key.code {
                KeyCode::Esc => {
                    app.modal = ModalState::None;
                }
                KeyCode::Enter => {
                    if !query.is_empty() {
                        app.pending_action = PendingAction::Search { query };
                        app.spinner_label = "Searching...".to_string();
                        app.modal = ModalState::Loading;
                    } else {
                        app.modal = ModalState::None;
                    }
                }
                KeyCode::Backspace => {
                    let mut q = query;
                    q.pop();
                    app.modal = ModalState::SearchInput { query: q };
                }
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.modal = ModalState::SearchInput {
                        query: query + &c.to_string(),
                    };
                }
                _ => {}
            }
        }

        ModalState::Find { query } => {
            let query = query.clone();
            match key.code {
                KeyCode::Esc => {
                    app.modal = ModalState::None;
                }
                KeyCode::Enter => {
                    app.modal = ModalState::None;
                    if !query.is_empty() {
                        app.last_search = Some(query.clone());
                        app.find_next_with(&query);
                    }
                }
                KeyCode::Backspace => {
                    let mut q = query;
                    q.pop();
                    app.modal = ModalState::Find { query: q };
                }
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.modal = ModalState::Find {
                        query: query + &c.to_string(),
                    };
                }
                _ => {}
            }
        }

        ModalState::None => match key.code {
            KeyCode::Char('q') => {
                app.app_state = AppState::Quitting;
            }
            KeyCode::Esc => {
                if app.breadcrumbs.len() > 1 {
                    app.go_back();
                    app.needs_reload = true;
                } else {
                    app.app_state = AppState::Quitting;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => app.move_up(),
            KeyCode::Down | KeyCode::Char('j') => app.move_down(),
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.move_page_up()
            }
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.move_page_down()
            }
            KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if let Some(file) = app.selected_file() {
                    app.modal = ModalState::FileActions {
                        file_id: file.id,
                        file_name: file.name.clone(),
                        file_type: file.file_type.clone(),
                        selected: 0,
                    };
                }
            }
            KeyCode::Enter => {
                if let Some(file) = app.selected_file() {
                    let file_id = file.id;
                    let file_name = file.name.clone();
                    let file_type = file.file_type.clone();
                    if file_type == "FOLDER" {
                        app.enter_folder(file_id, file_name);
                        app.needs_reload = true;
                    } else {
                        app.modal = ModalState::FileActions {
                            file_id,
                            file_name,
                            file_type,
                            selected: 0,
                        };
                    }
                }
            }
            KeyCode::Left | KeyCode::Backspace => {
                app.go_back();
                app.needs_reload = true;
            }
            KeyCode::Char('/') => {
                app.modal = ModalState::Find {
                    query: String::new(),
                };
            }
            KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.modal = ModalState::SearchInput {
                    query: String::new(),
                };
            }
            KeyCode::Char('n') => {
                app.find_next();
            }
            KeyCode::Char('s') => app.cycle_sort_field(),
            KeyCode::Char('r') => app.toggle_sort_direction(),
            KeyCode::Char('x') => {
                if let Some(file) = app.selected_file() {
                    let file_id = file.id;
                    let file_name = file.name.clone();
                    app.modal = ModalState::ConfirmDelete { file_id, file_name };
                }
            }
            _ => {}
        },
    }
}

fn execute_file_action(
    app: &mut BrowserApp,
    action: &str,
    file_id: i64,
    file_type: &str,
    api_token: &String,
    client: &Client,
) {
    match action {
        "Copy URL" => match put::files::url(client, api_token, file_id) {
            Ok(r) => copy_to_clipboard(app, &r.url, "URL copied!"),
            Err(e) => app.modal = ModalState::Error(format!("Failed to get URL: {}", e)),
        },
        "Copy Stream URL" => {
            let url = format!(
                "https://api.put.io/v2/files/{}/stream?oauth_token={}",
                file_id, api_token
            );
            copy_to_clipboard(app, &url, "Stream URL copied!");
        }
        "Download" => {
            app.pending_action = PendingAction::Download { file_id };
        }
        "Open in browser" => {
            open_in_browser(app, &format!("https://app.put.io/files/{}", file_id));
        }
        "Copy file ID" => {
            copy_to_clipboard(app, &file_id.to_string(), "File ID copied!");
        }
        "Copy path" => {
            let (file_name, parent_id) = match app.files.iter().find(|f| f.id == file_id) {
                Some(file) => (file.name.clone(), file.parent_id),
                None => {
                    app.modal = ModalState::Error("File not found for path lookup.".to_string());
                    return;
                }
            };
            app.pending_action = PendingAction::CopyPath {
                file_name,
                parent_id,
            };
            app.spinner_label = "Copying path...".to_string();
            app.modal = ModalState::Loading;
        }
        "Download as zip" => {
            app.pending_action = PendingAction::Download { file_id };
        }
        "Copy folder ID" => {
            copy_to_clipboard(app, &file_id.to_string(), "Folder ID copied!");
        }
        "Go to folder" => {
            let parent_id = app
                .files
                .iter()
                .find(|f| f.id == file_id)
                .map(|f| f.parent_id)
                .unwrap_or(0);
            app.pending_action = PendingAction::GoToFolder { parent_id, file_id };
        }
        _ => {}
    }
    let _ = file_type;
}

pub(super) fn build_path_parts(
    client: &Client,
    api_token: &String,
    mut parent_id: i64,
) -> Result<Vec<String>, String> {
    if parent_id < 0 {
        return Err("Path lookup failed: invalid parent id.".to_string());
    }

    let mut parts = Vec::new();
    let mut depth = 0;
    while parent_id != 0 {
        depth += 1;
        if depth > 256 {
            return Err("Path lookup failed: path too deep.".to_string());
        }

        let response = put::files::list(client, api_token, parent_id)
            .map_err(|e| format!("Path lookup failed: {}", e))?;
        let folder = response.parent;

        if folder.name.is_empty() {
            return Err("Path lookup failed: missing folder name.".to_string());
        }
        if folder.parent_id == parent_id {
            return Err("Path lookup failed: parent loop detected.".to_string());
        }

        parts.push(folder.name);
        parent_id = folder.parent_id;
    }

    parts.reverse();
    Ok(parts)
}

fn open_in_browser(app: &mut BrowserApp, url: &str) {
    let mut command = if cfg!(target_os = "macos") {
        let mut cmd = std::process::Command::new("open");
        cmd.arg(url);
        cmd
    } else if cfg!(target_os = "windows") {
        // Use the default browser on Windows via the shell
        let mut cmd = std::process::Command::new("cmd");
        cmd.args(["/C", "start", ""]);
        cmd.arg(url);
        cmd
    } else {
        // Fallback for Unix-like systems
        let mut cmd = std::process::Command::new("xdg-open");
        cmd.arg(url);
        cmd
    };

    match command.spawn() {
        Ok(_) => app.modal = ModalState::Success("Opening in browser...".to_string()),
        Err(e) => app.modal = ModalState::Error(format!("Could not open browser: {}", e)),
    }
}

pub(super) fn copy_to_clipboard(app: &mut BrowserApp, text: &str, success_msg: &str) {
    match arboard::Clipboard::new() {
        Ok(mut cb) => match cb.set_text(text) {
            Ok(_) => app.modal = ModalState::Success(success_msg.to_string()),
            Err(e) => app.modal = ModalState::Error(format!("Clipboard error: {}", e)),
        },
        Err(e) => app.modal = ModalState::Error(format!("Clipboard unavailable: {}", e)),
    }
}
