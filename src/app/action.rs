use super::state::App;
use crate::app::state::AppMode;
use crate::input::keymap::KeyMap;
use crossterm::event::{KeyCode, KeyEvent};
impl App {
    pub fn handle_event(&mut self, event: KeyEvent, keymap: &KeyMap) {
        // ⚠️ FIX: Check if an error exists first.
        // If the user hits Esc, clear the error and return early!
        if let Some(_) = &self.state.error {
            if event.code == KeyCode::Esc {
                self.state.error = None;
                return;
            }
        }

        match &mut self.state.mode {
            AppMode::Normal => {
                if let Some(action) = keymap.get(&event) {
                    self.state.error = None; // Reset any leftover error when performing a new action
                    self.execute_normal_action(*action);
                };
            }

            AppMode::Leader { menu } => {
                let old_menu = std::mem::replace(&mut self.state.mode, AppMode::Normal);
                if let AppMode::Leader { menu } = old_menu {
                    match event.code {
                        KeyCode::Esc => {}
                        KeyCode::Char(c) => {
                            self.execute_leader_action(menu, c);
                        }
                        _ => {}
                    }
                }
            }

            AppMode::Input {
                target: _,
                buffer,
                cursor,
            } => match event.code {
                KeyCode::Enter => {
                    self.execute_input_action();
                }
                KeyCode::Esc => {
                    // Reset to normal mode and wipe out any text in the buffer
                    self.state.mode = AppMode::Normal;
                    self.state.error = None;
                }
                KeyCode::Backspace => {
                    if *cursor > 0 {
                        buffer.remove(*cursor - 1);
                        *cursor -= 1;
                    }
                }

                KeyCode::Left => {
                    if *cursor > 0 {
                        *cursor -= 1;
                    }
                }
                KeyCode::Right => {
                    if *cursor < buffer.len() {
                        *cursor += 1;
                    }
                }
                KeyCode::Home => {
                    *cursor = 0;
                }
                KeyCode::End => {
                    *cursor = buffer.len();
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
                    self.state.mode = AppMode::Normal;
                    self.state.error = None;
                }
                _ => {}
            },
        }
    }

    pub fn move_cursor_up(&mut self) {
        let len = self.state.files.len();
        if len == 0 {
            return;
        }
        let next = match self.ui.list_state.selected() {
            Some(0) | None => len - 1, // wrap
            Some(i) => i - 1,
        };
        self.ui.list_state.select(Some(next));
    }

    /// Move cursor down, wrapping to the top.
    pub fn move_cursor_down(&mut self) {
        let len = self.state.files.len();
        if len == 0 {
            return;
        }
        let next = match self.ui.list_state.selected() {
            Some(i) if i + 1 >= len => 0, // wrap
            Some(i) => i + 1,
            None => 0,
        };
        self.ui.list_state.select(Some(next));
    }
}
