// ui/layout.rs
use ratatui::layout::{Constraint, Layout, Rect};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

pub fn main_layout(area: Rect) -> ( Rect, Rect, Rect, Rect) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(area);

    let horizontal = Layout::horizontal([ 
        Constraint::Percentage(50),   // current dir
        Constraint::Percentage(50),   // preview
    ])
    .split(vertical[1]);

    (vertical[0], horizontal[0], horizontal[1], vertical[2])
    // (parent,      current,       preview,        status)
}