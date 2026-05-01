// input/handler.rs
use crate::app::state::{App, Mode};
use crate::fs::explorer;
use crossterm::event::KeyCode;
use std::io;

pub fn handle(app: &mut App, key: KeyCode) -> io::Result<()> {
    // input popup mode
    if matches!(app.mode, Mode::NewFile | Mode::NewDir | Mode::Rename) {
        match key {
            KeyCode::Enter => handle_input_confirm(app)?,
            KeyCode::Esc => {
                app.mode = Mode::Normal;
                app.input_text.clear();
            }
            KeyCode::Char(c) => {
                app.input_text.insert(app.cursor_pos, c);
                app.cursor_pos -= 1
            }
            KeyCode::Backspace => {
                if app.cursor_pos > 0 {
                    app.input_text.remove(app.cursor_pos - 1);
                    app.cursor_pos -= 1;
                }
            }
            KeyCode::Left => {
                app.cursor_pos = app.cursor_pos.saturating_sub(1);
            }
            KeyCode::Right => {
                app.cursor_pos = (app.cursor_pos + 1).min(app.input_text.len());
            }
            _ => {}
        }
        return Ok(());
    }

    // delete confirm mode
    if matches!(app.mode, Mode::Delete) {
        match key {
            KeyCode::Char('y') => {
                explorer::delete(&app.path, &app.files, &app.state)?;
                app.reload();
                app.mode = Mode::Normal;
            }
            KeyCode::Char('n') => app.mode = Mode::Normal,
            _ => {}
        }
        return Ok(());
    }

    // normal mode
    match key {
        KeyCode::Down => app.move_down(),
        KeyCode::Up => app.move_up(),
        KeyCode::Right => app.enter_dir(),
        KeyCode::Left => app.go_up(),
        KeyCode::Char('.') => app.toggle_hidden(),

        KeyCode::Char('d') => app.mode = Mode::Delete,
        KeyCode::Char('r') => {
            app.mode = Mode::Rename;
            app.input_text = app.files[app.state.selected().unwrap_or(0)].clone();
        }
        KeyCode::Char('f') => app.mode = Mode::NewFile,
        KeyCode::Char('n') => app.mode = Mode::NewDir,
        KeyCode::Char('q') => app.should_quit = true,
        _ => {}
    }

    Ok(())
}

fn handle_input_confirm(app: &mut App) -> io::Result<()> {
    if app.input_text.is_empty() {
        return Ok(());
    }

    match app.mode {
        Mode::NewDir => {
            explorer::create_dir(&app.path, &app.input_text)?;
            app.reload();
        }
        Mode::NewFile => match explorer::create_file(&app.path, &app.input_text) {
            Ok(_) => {
                app.reload();
                app.error = None;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                app.error = Some(format!("'{}' already exists", app.input_text));
            }
            Err(e) => {
                app.error = Some(format!("Failed: {}", e));
            }
        },
        Mode::Rename => {
            explorer::rename(&app.files, &app.path, &app.input_text, &app.state)?;
            app.reload();
        }
        _ => {}
    }

    app.mode = Mode::Normal;
    app.input_text.clear();
    Ok(())
}
