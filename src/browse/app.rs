use crate::put::files::File;

#[derive(Clone, Copy, PartialEq)]
pub enum SortField {
    Name,
    Size,
    Date,
    Modified,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

pub struct BreadcrumbEntry {
    pub id: i64,
    pub name: String,
    pub saved_index: usize,
    pub saved_offset: usize,
}

pub enum AppState {
    Browsing,
    Quitting,
}

pub enum ModalState {
    None,
    Loading,
    ConfirmDelete {
        file_id: i64,
        file_name: String,
    },
    FileActions {
        file_id: i64,
        file_name: String,
        file_type: String,
        selected: usize,
    },
    Find {
        query: String,
    },
    SearchInput {
        query: String,
    },
    Error(String),
    Success(String),
}

pub struct FileAction {
    pub label: &'static str,
    pub key: char,
}

/// Returns the ordered list of actions available for a given file type.
/// Used by both the event handler and the UI renderer.
pub fn file_actions_for(file_type: &str, in_search_results: bool) -> Vec<FileAction> {
    let mut actions = if file_type == "FOLDER" {
        vec![
            FileAction {
                label: "Download as zip",
                key: 'z',
            },
            FileAction {
                label: "Open in browser",
                key: 'b',
            },
            FileAction {
                label: "Copy path",
                key: 'p',
            },
            FileAction {
                label: "Copy folder ID",
                key: 'i',
            },
        ]
    } else if file_type == "VIDEO" {
        vec![
            FileAction {
                label: "Copy URL",
                key: 'c',
            },
            FileAction {
                label: "Copy Stream URL",
                key: 's',
            },
            FileAction {
                label: "Download",
                key: 'd',
            },
            FileAction {
                label: "Open in browser",
                key: 'b',
            },
            FileAction {
                label: "Copy path",
                key: 'p',
            },
            FileAction {
                label: "Copy file ID",
                key: 'i',
            },
        ]
    } else {
        vec![
            FileAction {
                label: "Copy URL",
                key: 'c',
            },
            FileAction {
                label: "Download",
                key: 'd',
            },
            FileAction {
                label: "Open in browser",
                key: 'b',
            },
            FileAction {
                label: "Copy path",
                key: 'p',
            },
            FileAction {
                label: "Copy file ID",
                key: 'i',
            },
        ]
    };
    if in_search_results {
        actions.push(FileAction {
            label: "Go to folder",
            key: 'g',
        });
    }
    actions
}

pub enum PendingAction {
    None,
    Download { file_id: i64 },
    Search { query: String },
    GoToFolder { parent_id: i64, file_id: i64 },
    Delete { file_id: i64 },
    CopyPath { file_name: String, parent_id: i64 },
}

pub struct BrowserApp {
    pub current_folder_id: i64,
    pub breadcrumbs: Vec<BreadcrumbEntry>,
    pub files: Vec<File>,
    pub selected_index: usize,
    pub app_state: AppState,
    pub modal: ModalState,
    pub pending_action: PendingAction,
    pub sort_field: SortField,
    pub sort_direction: SortDirection,
    pub list_state: ratatui::widgets::ListState,
    restore_index: Option<usize>,
    restore_offset: Option<usize>,
    pub tick: u8,
    pub needs_reload: bool,
    pub spinner_label: String,
    pub last_search: Option<String>,
    pub is_search_results: bool,
    pub pending_select_id: Option<i64>,
}

impl BrowserApp {
    pub fn new() -> Self {
        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(Some(0));
        BrowserApp {
            current_folder_id: 0,
            breadcrumbs: vec![BreadcrumbEntry {
                id: 0,
                name: "My Files".to_string(),
                saved_index: 0,
                saved_offset: 0,
            }],
            files: vec![],
            selected_index: 0,
            app_state: AppState::Browsing,
            modal: ModalState::Loading,
            pending_action: PendingAction::None,
            sort_field: SortField::Name,
            sort_direction: SortDirection::Asc,
            list_state,
            restore_index: None,
            restore_offset: None,
            tick: 0,
            needs_reload: true,
            spinner_label: "Loading...".to_string(),
            last_search: None,
            is_search_results: false,
            pending_select_id: None,
        }
    }

    pub fn enter_folder(&mut self, id: i64, name: String) {
        // Save cursor and scroll position so we can restore them when going back
        if let Some(current) = self.breadcrumbs.last_mut() {
            current.saved_index = self.selected_index;
            current.saved_offset = *self.list_state.offset_mut();
        }
        self.breadcrumbs.push(BreadcrumbEntry {
            id,
            name,
            saved_index: 0,
            saved_offset: 0,
        });
        self.current_folder_id = id;
        self.files.clear();
        self.selected_index = 0;
        self.list_state.select(Some(0));
        self.modal = ModalState::Loading;
    }

    pub fn go_back(&mut self) {
        if self.breadcrumbs.len() > 1 {
            self.breadcrumbs.pop();
            self.is_search_results = false;
            let parent = self.breadcrumbs.last().unwrap();
            self.current_folder_id = parent.id;
            self.restore_index = Some(parent.saved_index);
            self.restore_offset = Some(parent.saved_offset);
            self.files.clear();
            self.selected_index = 0;
            self.list_state.select(Some(0));
            self.modal = ModalState::Loading;
        }
    }

    pub fn set_files(&mut self, files: Vec<File>) {
        self.files = files;
        self.sort_files();
        let (idx, apply_scroll) = if let Some(select_id) = self.pending_select_id.take() {
            let i = self
                .files
                .iter()
                .position(|f| f.id == select_id)
                .unwrap_or(0);
            (i, false) // let ratatui auto-scroll to the selected item
        } else {
            let i = self
                .restore_index
                .take()
                .unwrap_or(0)
                .min(self.files.len().saturating_sub(1));
            (i, true)
        };
        self.selected_index = idx;
        self.list_state.select(Some(idx));
        if apply_scroll {
            if let Some(offset) = self.restore_offset.take() {
                *self.list_state.offset_mut() = offset.min(self.files.len().saturating_sub(1));
            }
        } else {
            self.restore_offset = None;
        }
        self.modal = ModalState::None;
    }

    /// Display search results. Pushes a virtual breadcrumb (id = -1).
    /// If already showing search results, replaces them in-place.
    pub fn enter_search_results(&mut self, query: &str, files: Vec<File>) {
        if self.is_search_results {
            // Replace current search results without stacking breadcrumbs
            if let Some(crumb) = self.breadcrumbs.last_mut() {
                crumb.name = format!("Search: {}", query);
            }
        } else {
            // Save cursor position so going back restores it
            if let Some(current) = self.breadcrumbs.last_mut() {
                current.saved_index = self.selected_index;
                current.saved_offset = *self.list_state.offset_mut();
            }
            self.breadcrumbs.push(BreadcrumbEntry {
                id: -1,
                name: format!("Search: {}", query),
                saved_index: 0,
                saved_offset: 0,
            });
            self.is_search_results = true;
        }
        self.files = files;
        self.selected_index = 0;
        self.list_state.select(Some(0));
        *self.list_state.offset_mut() = 0;
        self.modal = ModalState::None;
    }

    /// Reset navigation back to root, clearing any search context.
    pub fn reset_to_root(&mut self) {
        self.breadcrumbs.truncate(1);
        self.breadcrumbs[0].saved_index = 0;
        self.breadcrumbs[0].saved_offset = 0;
        self.is_search_results = false;
        self.current_folder_id = 0;
    }

    /// Navigate directly to a folder and pre-select a file by id.
    pub fn navigate_to_folder(&mut self, parent_id: i64, file_id: i64) {
        self.reset_to_root();
        if parent_id != 0 {
            self.breadcrumbs.push(BreadcrumbEntry {
                id: parent_id,
                name: "...".to_string(), // updated from API response after load
                saved_index: 0,
                saved_offset: 0,
            });
            self.current_folder_id = parent_id;
        }
        self.pending_select_id = Some(file_id);
        self.files.clear();
        self.selected_index = 0;
        self.list_state.select(Some(0));
        self.modal = ModalState::Loading;
    }

    fn sort_files(&mut self) {
        let field = self.sort_field;
        let dir = self.sort_direction;
        self.files.sort_by(|a, b| {
            let ord = match field {
                SortField::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortField::Size => a.size.0.cmp(&b.size.0),
                SortField::Date => a.created_at.cmp(&b.created_at),
                SortField::Modified => a.updated_at.cmp(&b.updated_at),
            };
            if dir == SortDirection::Desc {
                ord.reverse()
            } else {
                ord
            }
        });
    }

    pub fn cycle_sort_field(&mut self) {
        self.sort_field = match self.sort_field {
            SortField::Name => SortField::Size,
            SortField::Size => SortField::Date,
            SortField::Date => SortField::Modified,
            SortField::Modified => SortField::Name,
        };
        self.sort_files();
        self.selected_index = 0;
        self.list_state.select(Some(0));
    }

    pub fn toggle_sort_direction(&mut self) {
        self.sort_direction = match self.sort_direction {
            SortDirection::Asc => SortDirection::Desc,
            SortDirection::Desc => SortDirection::Asc,
        };
        self.sort_files();
        self.selected_index = 0;
        self.list_state.select(Some(0));
    }

    pub fn selected_file(&self) -> Option<&File> {
        self.files.get(self.selected_index)
    }

    /// Preserve the current cursor and scroll position across the next reload.
    pub fn save_position_for_reload(&mut self) {
        self.restore_index = Some(self.selected_index);
        self.restore_offset = Some(*self.list_state.offset_mut());
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.list_state.select(Some(self.selected_index));
        }
    }

    pub fn move_down(&mut self) {
        if !self.files.is_empty() && self.selected_index < self.files.len() - 1 {
            self.selected_index += 1;
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Jump to the next file matching `query`, starting after the current selection.
    /// Wraps around. Returns true if a match was found.
    pub fn find_next_with(&mut self, query: &str) -> bool {
        if query.is_empty() || self.files.is_empty() {
            return false;
        }
        let q = query.to_lowercase();
        let n = self.files.len();
        for offset in 1..=n {
            let i = (self.selected_index + offset) % n;
            if self.files[i].name.to_lowercase().contains(&q) {
                self.selected_index = i;
                self.list_state.select(Some(i));
                return true;
            }
        }
        false
    }

    /// Repeat the last search.
    pub fn find_next(&mut self) -> bool {
        if let Some(query) = self.last_search.clone() {
            self.find_next_with(&query)
        } else {
            false
        }
    }

    pub fn move_page_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(10);
        self.list_state.select(Some(self.selected_index));
    }

    pub fn move_page_down(&mut self) {
        if !self.files.is_empty() {
            let last = self.files.len() - 1;
            self.selected_index = (self.selected_index + 10).min(last);
            self.list_state.select(Some(self.selected_index));
        }
    }
}
