// ui/components/file_list.rs
use ratatui::{Frame, prelude::*, symbols::border, widgets::{Block, Borders, List, ListItem}};
use crate::app::state::App;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app.files
        .iter()
        .map(|f| ListItem::new(f.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::new()
                .borders(Borders::LEFT)
                .border_set(border::LIGHT_DOUBLE_DASHED),
        )
        .highlight_spacing(ratatui::widgets::HighlightSpacing::WhenSelected)
        .highlight_style(Style::default().fg(Color::Blue).bg(Color::LightMagenta).add_modifier(Modifier::BOLD | Modifier::ITALIC));

    frame.render_stateful_widget(list, area, &mut app.state);
}