use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::state::AppMode, input::keymap::KeyMap};

use super::state::App;



impl App {
    pub fn handle_event(&mut self, event: KeyEvent, keymap: &KeyMap) {
        // ⚠️ FIX: Check if an error exists first. 
        // If the user hits Esc, clear the error and return early!
        if let Some(_) = &self.error {
            if event.code == KeyCode::Esc {
                self.error = None;
                return;
            }
        }

        match &mut self.mode {
            AppMode::Normal => {
                if let Some(action) = keymap.get(&event) {
                    self.error = None; // Reset any leftover error when performing a new action
                    self.execute_normal_action(*action);
                }
            }

            AppMode::Input { target:_, buffer, cursor } => match event.code {
                KeyCode::Enter => {
                    self.execute_input_action();
                }
                KeyCode::Esc => {
                    // Reset to normal mode and wipe out any text in the buffer
                    self.mode = AppMode::Normal;
                    self.error = None; 
                }
                KeyCode::Backspace => {
                    if *cursor > 0 {
                        buffer.remove(*cursor - 1);
                        *cursor -= 1;
                    }
                }
                KeyCode::Char(c) => {
                    buffer.insert(*cursor, c);
                    *cursor += 1;
                }
                _ => {}
            },

            AppMode::Confirm { .. } => match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.execute_confirm_action();
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.mode = AppMode::Normal;
                    self.error = None;
                }
                _ => {}
            },
        }
    }


    pub fn move_cursor_up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1 // Wrap to bottom
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn move_cursor_down(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
                    0 // Wrap to top
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}
