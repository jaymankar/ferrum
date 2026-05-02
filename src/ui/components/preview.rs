// ui/components/preview.rs
use ratatui::{Frame, prelude::*, symbols::border, widgets::{Block, Borders, Paragraph}};
use crate::app::state::App;
use crate::fs::explorer;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let preview_text = explorer::preview(
        &app.path,
        &app.list_state,
        &app.files,
        &app.settings.show_hidden,
    );

    let preview = Paragraph::new(preview_text)
        .block(
            Block::new()
                .borders(Borders::LEFT)
                .border_set(border::LIGHT_DOUBLE_DASHED)
        );

    frame.render_widget(preview, area);
}