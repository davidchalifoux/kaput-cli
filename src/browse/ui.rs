use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph},
    Frame,
};

use super::app::{file_actions_for, AppState, BrowserApp, FileAction, ModalState, SortField};

const MODAL_BG: Color = Color::Rgb(45, 45, 58);

pub fn draw(f: &mut Frame, app: &mut BrowserApp) {
    if matches!(app.app_state, AppState::Quitting) {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // breadcrumb
            Constraint::Min(0),    // file list
            Constraint::Length(2), // help bar
        ])
        .split(f.size());

    draw_breadcrumb(f, app, chunks[0]);
    draw_file_list(f, app, chunks[1]);
    draw_help_bar(f, app, chunks[2]);

    // Draw modal overlays last
    match &app.modal {
        ModalState::Loading => draw_spinner(f, app.tick, &app.spinner_label),
        ModalState::Error(msg) => draw_error_modal(f, msg.clone()),
        ModalState::Success(msg) => draw_success_modal(f, msg.clone()),
        ModalState::ConfirmDelete { file_name, .. } => draw_confirm_modal(f, file_name.clone()),
        ModalState::FileActions {
            file_name,
            file_type,
            selected,
            ..
        } => {
            draw_file_actions_modal(f, file_name, file_type, *selected, app.is_search_results);
        }
        ModalState::Find { query } => draw_find_bar(f, query),
        ModalState::SearchInput { query } => draw_search_input(f, query),
        ModalState::None => {}
    }
}

fn draw_breadcrumb(f: &mut Frame, app: &BrowserApp, area: Rect) {
    let crumb_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let sep_style = Style::default().fg(Color::DarkGray);

    let mut spans: Vec<Span> = vec![Span::raw(" ")];
    for (i, entry) in app.breadcrumbs.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ›  ", sep_style));
        }
        spans.push(Span::styled(truncate(&entry.name, 24), crumb_style));
    }

    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn draw_file_list(f: &mut Frame, app: &mut BrowserApp, area: Rect) {
    let search = app.last_search.clone();
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let cursor = if i == app.selected_index { ">>" } else { "  " };
            let color = file_type_color(&file.file_type);
            let is_folder = file.file_type == "FOLDER";
            let name_style = if is_folder {
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };
            let size_str = if is_folder {
                "—".to_string()
            } else {
                file.size.to_string()
            };
            let name_trunc = truncate(&file.name, 64);
            let padding = " ".repeat(64usize.saturating_sub(name_trunc.chars().count()) + 1);

            let mut spans = vec![Span::raw(format!("{} ", cursor))];
            if let Some(ref query) = search {
                let match_style = name_style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
                spans.extend(highlight_match(&name_trunc, query, name_style, match_style));
            } else {
                spans.push(Span::styled(name_trunc, name_style));
            }
            spans.push(Span::styled(padding, name_style));
            spans.push(Span::styled(
                format!("{:>10}", size_str),
                Style::default().fg(Color::DarkGray),
            ));

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(
            Style::default()
                .bg(Color::LightCyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn draw_help_bar(f: &mut Frame, app: &BrowserApp, area: Rect) {
    let k = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let l = Style::default().fg(Color::DarkGray);
    let sep = Span::styled("    ", l);

    let sort_label = match app.sort_field {
        SortField::Name => "Name    ",
        SortField::Size => "Size    ",
        SortField::Date => "Date    ",
        SortField::Modified => "Modified",
    };

    // 4 columns, 2 rows. Key right-aligned per column, label left-aligned.
    // Col 1: key_w=5  (↑↓/jk  /   Bksp)
    // Col 2: key_w=8  (Enter/^O  /  x)
    // Col 3: key_w=1  (s  /  r)
    // Col 4: key_w=5  (^U/^D  /  ^F)
    let row1 = Line::from(vec![
        Span::styled("↑↓", k),
        Span::styled("/", l),
        Span::styled("jk", k),
        Span::styled(format!("  {:<8}", "Navigate"), l),
        sep.clone(),
        Span::styled("Enter", k),
        Span::styled("/", l),
        Span::styled("^O", k),
        Span::styled(format!("  {:<6}", "Open"), l),
        sep.clone(),
        Span::styled("s", k),
        Span::styled(format!("  {:<8}", sort_label), l),
        sep.clone(),
        Span::styled("^U", k),
        Span::styled("/", l),
        Span::styled("^D", k),
        Span::styled(format!("  {:<6}", "Scroll"), l),
    ]);
    let row2 = Line::from(vec![
        Span::styled(format!("{:>5}", "Bksp"), k),
        Span::styled(format!("  {:<8}", "Back"), l),
        sep.clone(),
        Span::styled(format!("{:>8}", "x"), k),
        Span::styled(format!("  {:<6}", "Delete"), l),
        sep.clone(),
        Span::styled("r", k),
        Span::styled(format!("  {:<8}", "Reverse"), l),
        sep.clone(),
        Span::styled(format!("{:>5}", "^F"), k),
        Span::styled(format!("  {:<6}", "Search"), l),
    ]);

    f.render_widget(
        Paragraph::new(vec![row1, row2]).alignment(Alignment::Center),
        area,
    );
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: popup_width.min(r.width),
        height: height.min(r.height),
    }
}

fn draw_search_input(f: &mut Frame, query: &str) {
    let area = centered_rect(50, 5, f.size());
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::symmetric(2, 1))
        .title(" Search put.io ")
        .style(Style::default().fg(Color::Cyan).bg(MODAL_BG));
    let inner = block.inner(area);
    f.render_widget(block, area);
    f.render_widget(
        Paragraph::new(query).style(Style::default().fg(Color::White).bg(MODAL_BG)),
        inner,
    );
    let cursor_x =
        (inner.x + query.chars().count() as u16).min(inner.x + inner.width.saturating_sub(1));
    f.set_cursor(cursor_x, inner.y);
}

fn draw_find_bar(f: &mut Frame, query: &str) {
    let size = f.size();
    let y = size.height.saturating_sub(1);
    let area = Rect {
        x: 0,
        y,
        width: size.width,
        height: 1,
    };
    f.render_widget(Clear, area);
    let line = Line::from(vec![
        Span::styled(
            "/",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(query, Style::default().fg(Color::White)),
    ]);
    f.render_widget(Paragraph::new(line), area);
    // Place the real terminal cursor at the end of the query
    let cursor_x = (1 + query.chars().count() as u16).min(size.width.saturating_sub(1));
    f.set_cursor(cursor_x, y);
}

fn draw_spinner(f: &mut Frame, tick: u8, label: &str) {
    const FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let ch = FRAMES[tick as usize % FRAMES.len()];
    let size = f.size();
    let text = format!("{} {}", ch, label);
    let area = Rect {
        x: 1,
        y: size.height.saturating_sub(1),
        width: text.chars().count() as u16,
        height: 1,
    };
    f.render_widget(
        Paragraph::new(text).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        area,
    );
}

fn draw_error_modal(f: &mut Frame, msg: String) {
    let area = centered_rect(50, 7, f.size());
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::symmetric(2, 1))
        .title(" Error ")
        .style(Style::default().fg(Color::Red).bg(MODAL_BG));
    let inner = block.inner(area);
    f.render_widget(block, area);
    let p = Paragraph::new(format!("{}\n\nPress any key to dismiss", msg))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).bg(MODAL_BG));
    f.render_widget(p, inner);
}

fn draw_success_modal(f: &mut Frame, msg: String) {
    let area = centered_rect(40, 5, f.size());
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::symmetric(2, 1))
        .title(" Done ")
        .style(Style::default().fg(Color::Green).bg(MODAL_BG));
    let inner = block.inner(area);
    f.render_widget(block, area);
    let p = Paragraph::new(msg.as_str())
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green).bg(MODAL_BG));
    f.render_widget(p, inner);
}

fn draw_file_actions_modal(
    f: &mut Frame,
    file_name: &str,
    file_type: &str,
    selected: usize,
    in_search_results: bool,
) {
    let actions = file_actions_for(file_type, in_search_results);
    let height = actions.len() as u16 + 4; // borders + vertical padding
    let area = centered_rect(38, height, f.size());
    f.render_widget(Clear, area);

    let title = format!(" {} ", truncate(file_name, 64));
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::symmetric(1, 1))
        .title(title)
        .style(Style::default().bg(MODAL_BG));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let items: Vec<ListItem> = actions
        .iter()
        .enumerate()
        .map(|(i, FileAction { label, key })| {
            let is_sel = i == selected;
            let cursor = if is_sel { "▶" } else { " " };
            let (row_style, key_style) = if is_sel {
                let s = Style::default()
                    .bg(Color::LightCyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
                (s, s)
            } else {
                (
                    Style::default().bg(MODAL_BG),
                    Style::default().bg(MODAL_BG).fg(Color::DarkGray),
                )
            };
            let cursor_text = format!(" {} ", cursor);
            let key_text = format!("[{}] ", key);
            let label_text = label.to_string();
            let content_width =
                cursor_text.chars().count() + key_text.chars().count() + label_text.chars().count();
            let pad_width = inner.width.saturating_sub(content_width as u16) as usize;

            ListItem::new(Line::from(vec![
                Span::styled(cursor_text, row_style),
                Span::styled(key_text, key_style),
                Span::styled(label_text, row_style),
                // Fill the rest of the row so the highlight spans the full width
                Span::styled(" ".repeat(pad_width), row_style),
            ]))
        })
        .collect();

    f.render_widget(List::new(items), inner);
}

fn draw_confirm_modal(f: &mut Frame, file_name: String) {
    let area = centered_rect(50, 7, f.size());
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::symmetric(2, 1))
        .title(" Confirm Delete ")
        .style(Style::default().fg(Color::Yellow).bg(MODAL_BG));
    let inner = block.inner(area);
    f.render_widget(block, area);
    let p = Paragraph::new(format!("Delete \"{}\"?\n\n[y] Yes  [n] No", file_name))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow).bg(MODAL_BG));
    f.render_widget(p, inner);
}

fn file_type_color(file_type: &str) -> Color {
    match file_type {
        // Folders: bright warm yellow — visually dominant
        "FOLDER" => Color::LightYellow,
        // Files: standard (non-bright) colors, clearly subordinate to folders
        "VIDEO" => Color::Green,
        "AUDIO" => Color::Magenta,
        "IMAGE" => Color::Cyan,
        "ARCHIVE" => Color::Red,
        "PDF" => Color::Red,
        _ => Color::Gray,
    }
}

/// Splits `name` into up to three spans: text before the match, the matched
/// substring (styled with `highlight`), and text after. Falls back to a single
/// span with `base` style if no match is found.
fn highlight_match(name: &str, query: &str, base: Style, highlight: Style) -> Vec<Span<'static>> {
    if !query.is_empty() {
        let lower_name = name.to_lowercase();
        let lower_query = query.to_lowercase();
        if let Some(start) = lower_name.find(lower_query.as_str()) {
            let end = start + lower_query.len();
            if name.is_char_boundary(start) && name.is_char_boundary(end) {
                let mut spans = Vec::new();
                if start > 0 {
                    spans.push(Span::styled(name[..start].to_string(), base));
                }
                spans.push(Span::styled(name[start..end].to_string(), highlight));
                if end < name.len() {
                    spans.push(Span::styled(name[end..].to_string(), base));
                }
                return spans;
            }
        }
    }
    vec![Span::styled(name.to_string(), base)]
}

fn truncate(s: &str, max_chars: usize) -> String {
    let count = s.chars().count();
    if count <= max_chars {
        s.to_string()
    } else if max_chars == 0 {
        String::new()
    } else if max_chars == 1 {
        "…".to_string()
    } else {
        if let Some(dot) = s.rfind('.') {
            if dot > 0 && dot < s.len() - 1 {
                let (base, ext) = s.split_at(dot);
                let ext_chars = ext.chars().count();
                if ext_chars < max_chars {
                    let base_chars = max_chars - ext_chars - 1;
                    let base_trunc: String = base.chars().take(base_chars).collect();
                    return format!("{}…{}", base_trunc, ext);
                }
            }
        }

        let truncated: String = s.chars().take(max_chars.saturating_sub(1)).collect();
        format!("{}…", truncated)
    }
}
