// ui/components/parent_dir.rs
use ratatui::{Frame, prelude::*, symbols::border, widgets::{Block, Borders, List, ListItem}};
use crate::app::state::App;
use crate::fs::explorer;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    // get parent path
    let parent_path = app.path.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or(app.path.clone());

    // list parent dir contents
    let parent_files = explorer::list(&parent_path, &app.settings.show_hidden);

    // find current dir name to highlight it
    let current_name = app.path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let items: Vec<ListItem> = parent_files
        .iter()
        .map(|f| {
            if f == &current_name {
                // highlight current folder in parent panel
                ListItem::new(f.as_str())
                    .style(Style::default().fg(Color::Cyan))
            } else {
                ListItem::new(f.as_str())
            }
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::new()
                .borders(Borders::NONE)
                .border_set(border::LIGHT_DOUBLE_DASHED)
        );

    frame.render_widget(list, area);
}