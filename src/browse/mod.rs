mod app;
mod events;
mod ui;

use std::io;
use std::sync::mpsc;
use std::time::Duration;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use reqwest::blocking::Client;

use crate::put;
use app::{AppState, BrowserApp, ModalState, PendingAction};

pub fn run(client: &Client, api_token: &String) -> io::Result<()> {
    // Restore terminal on panic
    std::panic::set_hook(Box::new(|info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        eprintln!("{info}");
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = BrowserApp::new();

    loop {
        app.tick = app.tick.wrapping_add(1);
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if matches!(app.app_state, AppState::Quitting) {
            break;
        }

        if app.needs_reload {
            app.needs_reload = false;
            if !matches!(app.modal, ModalState::Loading) {
                app.spinner_label = "Loading...".to_string();
                app.modal = ModalState::Loading;
            }
            let client2 = client.clone();
            let token2 = api_token.clone();
            let folder_id = app.current_folder_id;
            let result = spin_while(&mut terminal, &mut app, move || {
                put::files::list(&client2, &token2, folder_id)
            })?;
            match result {
                Ok(r) => {
                    if app.current_folder_id != 0 {
                        if let Some(crumb) = app.breadcrumbs.last_mut() {
                            if crumb.id == app.current_folder_id {
                                crumb.name = r.parent.name.clone();
                            }
                        }
                    }
                    app.set_files(r.files);
                }
                Err(e) => app.modal = ModalState::Error(e.to_string()),
            }
            continue;
        }

        let pending = std::mem::replace(&mut app.pending_action, PendingAction::None);
        match pending {
            PendingAction::None => {}

            PendingAction::Search { query } => {
                let client2 = client.clone();
                let token2 = api_token.clone();
                let query2 = query.clone();
                let result = spin_while(&mut terminal, &mut app, move || {
                    put::files::search(&client2, &token2, &query2)
                })?;
                match result {
                    Ok(r) => app.enter_search_results(&query, r.files),
                    Err(e) => app.modal = ModalState::Error(format!("Search failed: {}", e)),
                }
            }

            PendingAction::GoToFolder { parent_id, file_id } => {
                app.navigate_to_folder(parent_id, file_id);
                app.needs_reload = true;
            }

            PendingAction::CopyPath {
                file_name,
                parent_id,
            } => {
                let client2 = client.clone();
                let token2 = api_token.clone();
                let result = spin_while(&mut terminal, &mut app, move || {
                    events::build_path_parts(&client2, &token2, parent_id)
                })?;
                match result {
                    Ok(mut parts) => {
                        parts.push(file_name);
                        let path = parts.join("/");
                        events::copy_to_clipboard(&mut app, &path, "Path copied!");
                    }
                    Err(e) => app.modal = ModalState::Error(e),
                }
            }

            PendingAction::Delete { file_id } => {
                let client2 = client.clone();
                let token2 = api_token.clone();
                let file_id_str = file_id.to_string();
                let result = spin_while(&mut terminal, &mut app, move || {
                    put::files::delete(&client2, &token2, &file_id_str)
                })?;
                match result {
                    Ok(_) => {
                        app.spinner_label = "Loading...".to_string();
                        app.needs_reload = true;
                    }
                    Err(e) => app.modal = ModalState::Error(format!("Delete failed: {}", e)),
                }
            }

            PendingAction::Download { file_id } => {
                disable_raw_mode()?;
                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                terminal.show_cursor()?;

                match put::files::download(client, api_token, file_id, false, None, false) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Download error: {}", e),
                }

                println!("\nPress Enter to return to the file browser...");
                let mut input = String::new();
                io::stdin().read_line(&mut input).ok();

                enable_raw_mode()?;
                execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                terminal.clear()?;
            }
        }

        if matches!(app.app_state, AppState::Quitting) {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                events::handle_key(&mut app, key, client, api_token);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

/// Runs a blocking closure on a background thread while keeping the TUI draw
/// loop alive so the spinner actually animates.
fn spin_while<T, F>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut BrowserApp,
    work: F,
) -> io::Result<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        tx.send(work()).ok();
    });
    loop {
        app.tick = app.tick.wrapping_add(1);
        terminal.draw(|f| ui::draw(f, app))?;
        match rx.try_recv() {
            Ok(result) => return Ok(result),
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err(io::Error::other("worker thread panicked"));
            }
            Err(mpsc::TryRecvError::Empty) => {
                std::thread::sleep(Duration::from_millis(80));
            }
        }
    }
}
