mod app;
mod ui;
mod fs;
mod input;
mod config;
mod utils;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app::run)?;
    Ok(())
}