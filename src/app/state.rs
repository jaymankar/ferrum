use crate::fs::explorer;
use core::fmt;
use ratatui::widgets::ListState;
use std::{io, path::PathBuf, time::SystemTime};

// ─────────────────────────────────────────────
//  Top-level App
// ─────────────────────────────────────────────

#[derive(Debug)]
pub struct App {
    pub state: AppState,
    pub ui: UiState,
}

// ─────────────────────────────────────────────
//  UI state  (view / interaction only)
// ─────────────────────────────────────────────

#[derive(Debug)]
pub struct UiState {
    /// Drives the ratatui List widget; single source of truth for selection.
    pub list_state: ListState,
}

// ─────────────────────────────────────────────
//  Application state  (business logic only)
// ─────────────────────────────────────────────

#[derive(Debug)]
pub struct AppState {
    // Navigation
    pub path: PathBuf,
    pub files: Vec<FileEntry>,

    // Settings
    pub settings: AppSettings,
    pub should_quit: bool,

    // Clipboard
    pub clipboard: Option<Clipboard>,

    //Sort
    pub sort:SortMode,

    // Mode system
    pub mode: AppMode,

    // Error handling
    pub error: Option<AppError>,
}

// ─────────────────────────────────────────────
//  Domain types
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub modify: SystemTime,
}

#[derive(Debug)]
pub struct Clipboard {
    pub paths: PathBuf,
    pub mode: ClipboardMode,
}

#[derive(Debug, PartialEq)]
pub enum ClipboardMode {
    Copy,
    // Paths are deleted on paste, not on copy.
    Cut,
}

#[derive(Debug)]
pub struct AppSettings {
    pub show_hidden: bool,
}

// ─────────────────────────────────────────────
//  Mode system
// ─────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub enum AppMode {
    Normal,

    Input {
        target: InputTarget,
        buffer: String,
        cursor: usize,
    },

    Confirm {
        target: ConfirmTarget,
        subject: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum SortMode{
    FileName,
    Size,
    Date,

}

#[derive(Debug, PartialEq, Eq)]
pub enum InputTarget {
    Rename,
    CreateFile,
    CreateDir,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfirmTarget {
    Delete { filename: String },
}

// ─────────────────────────────────────────────
//  Error system
// ─────────────────────────────────────────────

#[derive(Debug)]
pub enum AppError {
    FileExists(String),
    PermissionDenied(String),
    InvalidName(String),
    NotFound(String),
    Io(String),
}

impl AppError {
    /// Map a raw `io::Error` to a typed `AppError` using the name/path
    /// that was involved in the operation so error messages stay useful.
    pub fn from_io(err: io::Error, name: &str) -> Self {
        match err.kind() {
            io::ErrorKind::AlreadyExists => Self::FileExists(name.to_string()),
            io::ErrorKind::PermissionDenied => Self::PermissionDenied(name.to_string()),
            io::ErrorKind::NotFound => Self::NotFound(name.to_string()),
            _ => Self::Io(err.to_string()),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileExists(name) => write!(f, "'{name}' already exists"),
            Self::PermissionDenied(path) => write!(f, "Permission denied: {path}"),
            Self::InvalidName(name) => write!(f, "Invalid name: '{name}'"),
            Self::NotFound(path) => write!(f, "Not found: {path}"),
            Self::Io(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppError {}

// ─────────────────────────────────────────────
//  App implementation
// ─────────────────────────────────────────────

impl App {
    /// Create a new App, loading the current working directory.
    pub fn new() -> Self {
        let path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

        // Build with empty files; reload() fills them in.
        let mut app = Self {
            state: AppState {
                path,
                files: Vec::new(),
                settings: AppSettings { show_hidden: false },
                should_quit: false,
                clipboard: None,
                mode: AppMode::Normal,
                sort:SortMode::FileName,
                error: None,
            },
            ui: UiState {
                list_state: ListState::default(),
            },
        };

        app.reload();
        app
    }

    /// Reload directory listing.
    /// Preserves the cursor position where possible instead of always
    /// jumping back to the top.
    pub fn reload(&mut self) {
        let previous = self.ui.list_state.selected();

        self.state.files =
            explorer::list(&self.state.path, self.state.settings.show_hidden).unwrap_or_default();

        let new_index = match previous {
            // Keep cursor where it was if that index still exists.
            Some(i) if i < self.state.files.len() => i,
            // Clamp to last item if the list shrank.
            _ if !self.state.files.is_empty() => 0,
            // Directory is empty.
            _ => {
                self.ui.list_state.select(None);
                return;
            }
        };

        self.ui.list_state.select(Some(new_index));
    }

    /// Safe, single access point for the currently selected file.
    /// All action code goes through here — never index `files` directly.
    pub fn selected_file(&self) -> Option<&FileEntry> {
        let i = self.ui.list_state.selected()?;
        self.state.files.get(i)
    }
}
