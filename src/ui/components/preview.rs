use ratatui::{
    Frame,
    prelude::*,
    symbols::border,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use crate::app::state::App;
use crate::fs::explorer;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let preview_text = match app.selected_file() {
        Some(file) => explorer::preview(file, app.state.settings.show_hidden),
        None       => String::new(),
    };

    let preview = Paragraph::new(preview_text)
        .block(
            Block::new()
                .borders(Borders::LEFT)
                .border_set(border::ROUNDED),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(preview, area);
}