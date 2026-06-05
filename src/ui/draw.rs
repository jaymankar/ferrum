use super::components;
use super::layout;
use super::layout::centered_rect;
use crate::app::state::LeaderMenu;
use crate::app::state::{App, AppError, AppMode, ConfirmTarget, InputTarget};
use ratatui::{
    Frame,
    prelude::*,
    symbols::border,
    widgets::{Block, Clear, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let (status_area_up, file_area, preview_area, status_area_bottom) =
        layout::main_layout(frame.area());

    components::status_bar_up::draw(frame, app, status_area_up);
    // components::parent_dir::draw(frame, app, parent_area);
    components::file_list::draw(frame, app, file_area);
    components::preview::draw(frame, app, preview_area);
    components::status_bar::draw(frame, app, status_area_bottom);

    match &app.state.mode {
        AppMode::Confirm { .. } => draw_delete_popup(frame, app),
        AppMode::Input { .. } => draw_input_popup(frame, app),
        AppMode::Normal => {
            if let Some(ref err) = app.state.error {
                draw_error_popup(frame, err);
            }
        }
        AppMode::Leader { .. } => draw_leader_popup(frame, app),
    }
}

fn draw_delete_popup(frame: &mut Frame, app: &App) {
    let filename = match &app.state.mode {
        AppMode::Confirm {
            target: ConfirmTarget::Delete { filename },
            ..
        } => filename.as_str(),
        _ => "unknown",
    };

    let area = centered_rect(50, 30, frame.area());
    let text = format!("Delete '{filename}'?\n\n(y)es  (n)o");

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
    let AppMode::Input {
        target,
        buffer,
        cursor,
    } = &app.state.mode
    else {
        return;
    };

    let title = match target {
        InputTarget::CreateFile => " Create File ",
        InputTarget::CreateDir => " Create Directory ",
        InputTarget::Rename => " Rename ",
    };

    let prefix = "Name: ";
    let display = format!("{prefix}{buffer}");

    let area = centered_rect(60, 20, frame.area());

    let popup = Paragraph::new(display)
        .block(Block::bordered().title(title).border_set(border::ROUNDED))
        .style(Style::default().fg(Color::Green));

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);

    // place the real terminal cursor at the correct position
    frame.set_cursor_position(Position {
        // area.x + 1 skips the left border
        // prefix.len() is "Name: " offset
        x: area.x + 1 + prefix.len() as u16 + *cursor as u16,
        // area.y + 1 skips the top border
        y: area.y + 1,
    });
}

fn draw_error_popup(frame: &mut Frame, error: &AppError) {
    let area = centered_rect(50, 25, frame.area());
    let text = format!("Error:\n{error}\n\nPress Esc to dismiss");

    let popup = Paragraph::new(text)
        .block(
            Block::bordered()
                .title(" !! Error !! ")
                .border_set(border::ROUNDED),
        )
        .style(Style::default().fg(Color::LightRed))
        .centered();

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}
fn draw_leader_popup(frame: &mut Frame, app: &App) {
    let AppMode::Leader { menu } = &app.state.mode else {
        return;
    };
    let (title, hints) = match menu {
        LeaderMenu::Sort => (" Sort ", "n → name    s → size    d → date"),
    };

    let area = centered_rect(45, 15, frame.area());
    let popup = Paragraph::new(hints)
        .block(Block::bordered().title(title).border_set(border::ROUNDED))
        .style(Style::default().fg(Color::Yellow))
        .centered();

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}
