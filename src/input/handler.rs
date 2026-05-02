use crate::{
    app::state::{App, AppError, AppMode, ConfirmTarget, InputTarget},
    fs::explorer,
    input::keymap::Action,
};

impl App {
    pub fn execute_input_action(&mut self) {
        // Take the current mode out so we can read its data
        let current_mode = std::mem::replace(&mut self.mode, AppMode::Normal);

        if let AppMode::Input { target, buffer, .. } = current_mode {
            // Evaluate the match to return the exact same type from all arms
            let result = match target {
                InputTarget::Rename => {
                    explorer::rename(&self.files, &self.path, &buffer, &self.list_state)
                }
                InputTarget::CreateFile => {
                    // ⚠️ Removed the semicolon at the end
                    explorer::create_file(&self.path, &buffer)
                }
                InputTarget::CreateDir => {
                    // ⚠️ Removed the semicolon at the end
                    explorer::create_dir(&self.path, &buffer)
                }
            };

            // Now both 'Ok' and 'Err' variants can be handled perfectly!
            match result {
                Ok(_) => {
                    self.reload(); // Refresh screen files
                    self.error = None;
                }
                Err(err) => {
                    self.error = Some(AppError::Io(err.to_string()));
                }
            }
        }
    }
    pub fn execute_confirm_action(&mut self) {
        // Remove the current mode from self to safely extract its data
        let current_mode = std::mem::replace(&mut self.mode, AppMode::Normal);

        if let AppMode::Confirm { target, subject: _ } = current_mode {
            match target {
                ConfirmTarget::Delete { filename: _ } => {
                    // Execute the file deletion logic
                    match explorer::delete(&self.path, &self.files, &self.list_state) {
                        Ok(_) => {
                            self.reload();

                            // Adjust list state if deleting the last item in the folder
                            if let Some(selected) = self.list_state.selected() {
                                if selected >= self.files.len() && !self.files.is_empty() {
                                    self.list_state.select(Some(self.files.len() - 1));
                                } else if self.files.is_empty() {
                                    self.list_state.select(None);
                                }
                            }
                            self.error = None;
                        }
                        Err(err) => {
                            self.error = Some(AppError::Io(err.to_string()));
                        }
                    }
                }
            }
        }
    }

    pub fn execute_normal_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }
            Action::MoveUp => {
                self.move_cursor_up();
            }
            Action::MoveDown => {
                self.move_cursor_down();
            }
            Action::MoveBack => {
                self.path = explorer::go_to_parent(&self.path);
                self.reload();
                self.list_state.select(Some(0));
            }
            Action::MoveForward => {
                if let Some(i) = self.list_state.selected() {
                    let selectesd = self.path.join(&self.files[i]);
                    if selectesd.is_dir() {
                        self.path = selectesd;
                        self.files = explorer::list(&self.path, &self.settings.show_hidden);
                        self.list_state.select(Some(0));
                    }
                }
            }
            Action::HiddenFile => {
                self.settings.show_hidden = !self.settings.show_hidden;
                self.reload();
            }

           Action::Rename => {
                // 1. Correctly get the selected index from ListState
                if let Some(index) = self.list_state.selected() {
                    // 2. Safely extract that file from your files list
                    if let Some(name) = self.files.get(index) {
                        
                        // 3. Switch to Input mode with the EXACT name of the selected file!
                        self.mode = AppMode::Input {
                            target: InputTarget::Rename,
                            buffer: name.clone(), // This populates the buffer with the file name
                            cursor: name.len(),   // This puts the cursor at the very end of the word
                        };
                    }
                }
            }
            
        
            Action::Delete => {
                if let Some(filename) = self.get_selected_filename() {
                    // Switch to Confirm mode to ask the user "Are you sure?"
                    self.mode = AppMode::Confirm {
                        target: ConfirmTarget::Delete {
                            filename: filename.clone(),
                        },
                        subject: format!("Delete '{}'?", filename),
                    };
                }
            }

            Action::CreateDir => {
                // Switch to Input mode with an empty buffer and zeroed cursor
                self.mode = AppMode::Input {
                    target: InputTarget::CreateDir,
                    buffer: String::new(),
                    cursor: 0,
                };
            }
            Action::CreateFile => {
                self.mode = AppMode::Input {
                    target: InputTarget::CreateFile,
                    buffer: String::new(),
                    cursor: 0,
                };
            }
    
            _ => {}
        }
    }
}
