// ui/components/status_bar.rs
use ratatui::{Frame, prelude::*, widgets::{Paragraph}};
use crate::app::state::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let status = if let Some(err) = &app.error {
        Paragraph::new(format!("❌ {}", err))
            .style(Style::default().fg(Color::White))
    } else {
        Paragraph::new(format!(
            " {}",
            app.path.display(),

        ))
        .style(Style::default().fg(Color::Red))
    };

    frame.render_widget(status, area);
}