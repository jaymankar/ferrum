use ratatui::{
    Frame,
    prelude::*,
    symbols::border,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem},
};
use crate::app::state::{App, ClipboardMode};
use crate::ui::icons::get_icon;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app.state.files
        .iter()
        .map(|f| {
            let label = format!("{} {}", get_icon(f), f.name);

            let is_cut = app.state.clipboard
                .as_ref()
                .map(|c| c.mode == ClipboardMode::Cut && c.paths == f.path)
                .unwrap_or(false);

            let style = if is_cut {
                Style::default().add_modifier(Modifier::DIM)
            } else {
                Style::default()
            };

            ListItem::new(label).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::new()
                .borders(Borders::LEFT)
                .border_set(border::ROUNDED),
        )
        .highlight_spacing(HighlightSpacing::WhenSelected)
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(220, 220, 220))
                .bg(Color::Rgb(45, 45, 45))
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(list, area, &mut app.ui.list_state);
}