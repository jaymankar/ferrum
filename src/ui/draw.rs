// ui/draw.rs
use super::components;
use super::layout;
use super::layout::centered_rect;
use crate::app::state::{App, AppMode, InputTarget};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Clear, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let (status_area_up, parent_area, file_area, preview_area, status_area_bottom) = 
        layout::main_layout(frame.area());

    // 1. Draw your core components first
    components::status_bar_up::draw(frame, app, status_area_up);
    components::parent_dir::draw(frame, app, parent_area);
    components::file_list::draw(frame, app, file_area);
    components::preview::draw(frame, app, preview_area);
    components::status_bar::draw(frame, app, status_area_bottom);

    // 2. Draw any popups or modals on top of the components
    match &app.mode {
        AppMode::Confirm { .. } => {
            draw_delete_popup(frame, app);
        }
        AppMode::Input { .. } => {
            draw_input_popup(frame, app);
        }
        AppMode::Normal => {
            // 3. If there's an error in normal mode, render it as a popup
            if let Some(ref err) = app.error {
                draw_error_popup(frame, err.to_string());
            }
        }
    }
}

fn draw_delete_popup(frame: &mut Frame, app: &App) {
    let filename = app
        .list_state
        .selected()
        .and_then(|i| app.files.get(i))
        .map(|s| s.as_str())
        .unwrap_or("unknown");

    let area = centered_rect(50, 30, frame.area());
    let text = format!("Delete '{}'?\n\n(y)es  (n)o", filename);
    let popup = Paragraph::new(text)
        .block(
            Block::bordered()
                .title(" Confirm Delete ")
                .border_set(border::HEAVY_DOUBLE_DASHED),
        )
        .style(Style::default().fg(Color::Red))
        .centered();

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

fn draw_input_popup(frame: &mut Frame, app: &App) {
    let title = match &app.mode {
        AppMode::Input { target, .. } => match target {
            InputTarget::CreateFile => " Create File ",
            InputTarget::CreateDir => " Create Directory ",
            InputTarget::Rename { .. } => " Rename ",
        },
        _ => " Input ",
    };

    if let AppMode::Input { buffer, cursor, .. } = &app.mode {
        let cursor_pos = *cursor;
        let before = &buffer[..cursor_pos];
        let after = &buffer[cursor_pos..];

        let area = centered_rect(60, 20, frame.area());
        let prompt = format!("Name: {}|{}", before, after);
        
        let popup = Paragraph::new(prompt)
            .block(Block::bordered().title(title).border_set(border::ROUNDED))
            .style(Style::default().fg(Color::Green));

        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
    }
}

// ✅ 4. Dedicated error modal popup
fn draw_error_popup(frame: &mut Frame, error_msg: String) {
    let area = centered_rect(50, 25, frame.area());
    let text = format!("Error:\n{}\n\nPress 'Esc' to dismiss", error_msg);
    
    let popup = Paragraph::new(text)
        .block(
            Block::bordered()
                .title(" !! System Alert !! ")
                .border_set(border::ROUNDED),
        )
        .style(Style::default().fg(Color::LightRed))
        .centered();

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}