// ui/draw.rs
use super::components;
use super::layout;
use super::layout::centered_rect;
use crate::app::state::{App, Mode};
use ratatui::{
    Frame,
    prelude::*,
    symbols::border,
    widgets::{Block, Clear, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let (status_area_up,parent_area, file_area, preview_area, status_area_bottom) = layout::main_layout(frame.area());
    components::status_bar_up::draw(frame, app, status_area_up);
    components::parent_dir::draw(frame, app, parent_area); // new
    components::file_list::draw(frame, app, file_area);
    components::preview::draw(frame, app, preview_area);
    components::status_bar::draw(frame, app,status_area_bottom );

    match app.mode {
        Mode::Delete => draw_delete_popup(frame, app),
        Mode::NewFile | Mode::NewDir | Mode::Rename => draw_input_popup(frame, app),
        Mode::Normal => {}
    }
}

fn draw_delete_popup(frame: &mut Frame, app: &App) {
    let filename = app
        .state
        .selected()
        .and_then(|i| app.files.get(i))
        .map(|s| s.as_str())
        .unwrap_or("unknown");

    let area = centered_rect(50, 30, frame.area());
    let text = format!("Delete '{}'?\n\n(y)es  (n)o", filename);
    let popup = Paragraph::new(text)
        .block(
            Block::bordered()
                .title("Confirm Delete")
                .border_set(border::HEAVY_DOUBLE_DASHED),
        )
        .style(Style::default().fg(Color::Red))
        .centered();

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

fn draw_input_popup(frame: &mut Frame, app: &App) {
    let title = match app.mode {
        Mode::NewFile => "Create File",
        Mode::NewDir => "Create Directory",
        Mode::Rename => "Rename",
        _ => "Input",
    };
    let before = &app.input_text[..app.cursor_pos];
    let after = &app.input_text[app.cursor_pos..];

    let area = centered_rect(60, 20, frame.area());
    let prompt = format!("Name: {}|{}", before, after);
    let popup = Paragraph::new(prompt)
        .block(Block::bordered().title(title).border_set(border::ROUNDED))
        .style(Style::default().fg(Color::Green));

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}
