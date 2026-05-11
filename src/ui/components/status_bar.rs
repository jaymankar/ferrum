// ui/components/status_bar.rs
use ratatui::{Frame, prelude::*, widgets::{Paragraph}};
use crate::app::state::{ App};
pub fn draw(frame: &mut Frame, app: &App, area: Rect) {

    let status = if let Some(err) = &app.state.error {
        Paragraph::new(format!("❌ {}", err))
            .style(Style::default().fg(Color::White))
    } else {
        Paragraph::new(format!(
            " | {} items {}",
            // app.files.display(),
            app.state.files.len(),
            if app.state.settings.show_hidden { "| hidden: on" } else { "" }
        ))
        .style(Style::default().fg(Color::White))
    };

    frame.render_widget(status, area);
}