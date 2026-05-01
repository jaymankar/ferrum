// app/app.rs
use ratatui::DefaultTerminal;
use std::io;
use crate::app::state::App;
use crate::ui;
use crate::input;

pub fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            input::handle(&mut app, key.code)?;
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}