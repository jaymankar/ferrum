
// app/state.rs
use ratatui::widgets::ListState;
use std::path::PathBuf;
use crate::fs::explorer;

pub enum Mode {
    Normal,
    Delete,
    Rename,
    NewFile,
    NewDir,
    // ConfirmOverwrite,
}

pub struct App {
    pub path: PathBuf,
    pub files: Vec<String>,
    pub state: ListState,
    pub show_hidden: bool,
    pub mode: Mode,
    pub input_text: String,
    pub cursor_pos: usize,
    pub error: Option<String>,
    // pub yanked: Option<PathBuf>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let path = PathBuf::from("/home/jay/");
        let show_hidden = false;
        let files = explorer::list(&path, &show_hidden);
        let state = ListState::default().with_selected(Some(0));
        let cursor_pos = 0;

        Self {
            path,
            files,
            state,
            show_hidden,
            mode: Mode::Normal,
            input_text: String::new(),
            cursor_pos,
            error: None,
            // yanked: None
            should_quit: false,
        }
    }

    pub fn reload(&mut self) {
        self.files = explorer::list(&self.path, &self.show_hidden);
    }
}