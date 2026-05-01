// app/action.rs
use crate::app::state::App;

impl App {

    pub fn move_up(&mut self) {
        let i = self.state.selected().unwrap_or(0);
        self.state.select(Some(i.saturating_sub(1)));
    }

    pub fn move_down(&mut self) {
        let i = self.state.selected().unwrap_or(0);
        self.state.select(Some((i + 1).min(self.files.len() - 1)));
    }

    pub fn enter_dir(&mut self) {
        if let Some(i) = self.state.selected() {
            let selected = self.path.join(&self.files[i]);
            if selected.is_dir() {
                self.path = selected;
                self.reload();
                self.state.select(Some(0));
            }
        }
    }

    pub fn go_up(&mut self) {
        if let Some(parent) = self.path.parent() {
            self.path = parent.to_path_buf();
            self.reload();
            self.state.select(Some(0));
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.reload();
    }

    // pub fn selected_path(&self) -> Option<std::path::PathBuf> {
    //     self.state.selected()
    //         .and_then(|i| self.files.get(i))
    //         .map(|f| self.path.join(f))
    // }
}