use crate::{
    app::state::{App, AppError, AppMode, Clipboard, ClipboardMode, ConfirmTarget, InputTarget},
    fs::explorer,
    input::keymap::Action,
};

impl App {
    // ─────────────────────────────────────────────
    //  Input mode  (Rename / CreateFile / CreateDir)
    // ─────────────────────────────────────────────

    pub fn execute_input_action(&mut self) {
        let current_mode = std::mem::replace(&mut self.state.mode, AppMode::Normal);

        if let AppMode::Input { target, buffer, .. } = current_mode {
            let result: std::io::Result<()> = match target {
                InputTarget::Rename => {
                    if let Some(file) = self.selected_file() {
                        explorer::rename(file, &buffer)
                    } else {
                        Ok(()) // nothing selected — no-op
                    }
                }
                InputTarget::CreateFile => explorer::create_file(&self.state.path, &buffer),
                InputTarget::CreateDir => explorer::create_dir(&self.state.path, &buffer),
            };

            match result {
                Ok(_) => {
                    self.reload();
                    self.state.error = None;
                }
                Err(err) => {
                    // Use typed error so the UI can show a meaningful message.
                    self.state.error = Some(AppError::from_io(err, &buffer));
                }
            }
        }
    }

    // ─────────────────────────────────────────────
    //  Confirm mode  (Delete)
    // ─────────────────────────────────────────────

    pub fn execute_confirm_action(&mut self) {
        let current_mode = std::mem::replace(&mut self.state.mode, AppMode::Normal);

        if let AppMode::Confirm { target, .. } = current_mode {
            match target {
                ConfirmTarget::Delete { filename } => {
                    // Clone the path before the borrow ends with selected_file.
                    let file = match self.selected_file() {
                        Some(f) => f.clone(),
                        None => return,
                    };

                    match explorer::delete(&file) {
                        Ok(_) => {
                            self.reload();

                            // Clamp cursor if we deleted the last item.
                            if let Some(selected) = self.ui.list_state.selected() {
                                if self.state.files.is_empty() {
                                    self.ui.list_state.select(None);
                                } else if selected >= self.state.files.len() {
                                    self.ui.list_state.select(Some(self.state.files.len() - 1));
                                }
                            }

                            self.state.error = None;
                        }
                        Err(err) => {
                            self.state.error = Some(AppError::from_io(err, &filename));
                        }
                    }
                }
            }
        }
    }

    // ─────────────────────────────────────────────
    //  Normal mode
    // ─────────────────────────────────────────────

    pub fn execute_normal_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.state.should_quit = true;
            }

            Action::MoveUp => self.move_cursor_up(),
            Action::MoveDown => self.move_cursor_down(),

            Action::MoveBack => {
                self.state.path = explorer::go_to_parent(&self.state.path);
                self.reload();
            }

            Action::MoveForward => {
                if let Some(file) = self.selected_file() {
                    if file.is_dir {
                        self.state.path = file.path.clone();
                        self.reload();
                    }
                }
            }

            Action::HiddenFile => {
                self.state.settings.show_hidden = !self.state.settings.show_hidden;
                self.reload();
            }

            // Open Input mode pre-filled with the current filename.
            // The actual rename happens in execute_input_action.
            Action::Rename => {
                if let Some(file) = self.selected_file() {
                    let name = file.name.clone();
                    self.state.mode = AppMode::Input {
                        target: InputTarget::Rename,
                        buffer: name.clone(),
                        cursor: name.len(), // cursor at end of name
                    };
                }
            }

            Action::Delete => {
                if let Some(file) = self.selected_file() {
                    let name = file.name.clone();
                    self.state.mode = AppMode::Confirm {
                        target: ConfirmTarget::Delete {
                            filename: name.clone(),
                        },
                        subject: format!("Delete '{name}'?"),
                    };
                }
            }

            Action::CreateDir => {
                self.state.mode = AppMode::Input {
                    target: InputTarget::CreateDir,
                    buffer: String::new(),
                    cursor: 0,
                };
            }

            Action::CreateFile => {
                self.state.mode = AppMode::Input {
                    target: InputTarget::CreateFile,
                    buffer: String::new(),
                    cursor: 0,
                };
            }

            Action::Copy => {
                if let Some(file) = self.selected_file() {
                    self.state.clipboard = Some(Clipboard {
                        paths: file.path.clone(),
                        mode: ClipboardMode::Copy,
                    });
                }
                self.state.error = None;
            }

            Action::Cut => {
                if let Some(file) = self.selected_file() {
                    self.state.clipboard = Some(Clipboard {
                        paths: file.path.clone(),
                        mode: ClipboardMode::Cut,
                    });
                    self.state.error = None;
                };
            }

            Action::Paste => {
                let Some(clip) = &self.state.clipboard else {
                    return;
                };

                // let clip = self.state.clipboard;
                let src = clip.paths.clone();
                let is_cut = clip.mode == ClipboardMode::Cut;
                let dst = self.state.path.clone();

                let reslut = if is_cut {
                    explorer::move_item(&src, &dst)
                } else {
                    explorer::copy_item(&src, &dst)
                };

                match reslut {
                    Ok(_) => {
                        if is_cut {
                            self.state.clipboard = None;
                        }

                        self.reload();
                        self.state.error = None;
                    }

                    Err(err) => {
                        self.state.error = Some(AppError::from_io(
                            err,
                            src.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .as_ref(),
                        ));
                    }
                }
            }

            _ => {}
        }
    }
}
