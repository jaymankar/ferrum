// app/app.rs
use ratatui::DefaultTerminal;
use std::io;
use crate::app::state::{App, AppMode};
use crate::ui;
use crate::input::keymap::default_keymap;

pub fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
    let keymap = default_keymap(); // 1. Load the default keymap at start

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // 2. Pass the entire KeyEvent to your App's event handler
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            // Ignore key releases to prevent double handling on some systems
            if key.kind == crossterm::event::KeyEventKind::Press {
                app.handle_event(key, &keymap);
            }
        }

        // 3. Check if the user opted to quit
        if matches!(app.mode, AppMode::Normal) && app.should_quit {
            break;
        }
        
        // Alternatively, if you're using an AppMode::Quit variant:
        // if matches!(app.mode, AppMode::Quit) {
        //     break;
        // }
    }

    Ok(())
}