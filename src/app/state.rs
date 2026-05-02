// app/state.rs
use crate::fs::explorer;
use ratatui::widgets::ListState;
use core::fmt;
use std::path::PathBuf;

pub struct App {
    // Navigation state
    pub path: PathBuf,
    pub files: Vec<String>,
    pub list_state: ListState,
    pub selection: Selection,

    // Settings
    pub settings: AppSettings,
    pub should_quit:bool,

    // Mode with context
    pub mode: AppMode,

    // Error
    pub error: Option<AppError>,
}

pub enum Selection {
    None,
    Single(usize),
    Multiple(Vec<usize>), // for future bulk operations
}

pub struct AppSettings {
    pub show_hidden: bool,
    // pub sort_by: SortMode, // ← add this
}

// pub enum SortMode {
//     Name,
//     Size,
//     Date,
// }

// ✅ Each mode carries its own data
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
        subject: String, // ← ADDED: what are we confirming?
    },

}

#[derive(Debug, PartialEq, Eq)]
pub enum InputTarget {
    Rename,
    CreateFile,
    CreateDir,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfirmTarget {
    Delete {
        filename: String, // ← ADDED: deleting what?
    },
}

// ✅ Proper error type

pub enum AppError {
    FileExists(String),
    PermissionDenied(String),
    InvalidName(String),
    NotFound(String),
    Io(String),
}

// 1. Convert your error variants into printable strings
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileExists(name) => write!(f, "'{}' already exists", name),
            Self::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            Self::InvalidName(name) => write!(f, "Invalid name: '{}'", name),
            Self::NotFound(path) => write!(f, "Not found: {}", path),
            Self::Io(msg) => write!(f, "{}", msg),
        }
    }
}

// 2. Add Debug (Required for standard Error trait)
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppError: {}", self)
    }
}

// 3. Finally, implement the actual Error trait
impl std::error::Error for AppError {}

impl App {
    pub fn get_selected_filename(&self) -> Option<&String> {
        match self.selection {
            Selection::Single(i) if i < self.files.len() => Some(&self.files[i]),
            _ => None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        let path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let show_hidden_files = false;
        let should_quit = false;

        let files = explorer::list(&path, &show_hidden_files);
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            path,
            files,
            list_state,
            should_quit,
            selection: Selection::Single(0),
            settings: AppSettings {
                show_hidden: show_hidden_files,
                // sort_by: SortMode::Name,
            
            },

            
            mode: AppMode::Normal,
            error: None,
        }
    }

    pub fn reload(&mut self) {
        self.files = explorer::list(&self.path, &self.settings.show_hidden);
    }
}
