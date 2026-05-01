// fs/explorer.rs
use std::{fs, io, path::PathBuf};

use ratatui::widgets::ListState;

pub fn list(path: &PathBuf, show_hidden: &bool) -> Vec<String> {
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|f| f.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|name| {
                if *show_hidden {
                    true
                } else {
                    !name.starts_with('.')
                }
            })
            .collect(),
        Err(e) => {
            vec![format!("[Error: {}]", e)] // ✅ show error in list
        }
    }
}

pub fn delete(path: &PathBuf, files: &Vec<String>, state: &ListState) -> io::Result<()> {
    let i = match state.selected() {
        Some(index) => index,
        None => return Ok(()),
    };

    if files.is_empty() || i >= files.len() {
        return Ok(());
    }

    let select = path.join(&files[i]);

    if !select.exists() {
        return Ok(());
    }

    trash::delete(&select).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}

pub fn rename(
    files: &Vec<String>,
    path: &PathBuf,
    name: &str,
    state: &ListState,
) -> io::Result<()> {
    let index = match state.selected() {
        Some(index) => index,
        None => return Ok(()),
    };
    if files.is_empty() || index >= files.len() {
        return Ok(());
    }

    let old_path = path.join(&files[index]);
    let new_path = path.join(name);
    fs::rename(old_path, new_path)?;
    Ok(())
}
pub fn create_dir(path: &PathBuf, name: &str) -> io::Result<()> {
    let new_dir = path.join(name);
    fs::create_dir_all(&new_dir)?;

    Ok(())
}
pub fn create_file(path: &PathBuf, name: &str) -> io::Result<()> {
    let new_file = path.join(name);
    fs::File::create_new(&new_file)?;
    Ok(())
}

pub fn preview(
    path: &PathBuf,
    state: &ListState,
    files: &Vec<String>,
    show_hidden: &bool,
) -> String {
    let i = state.selected().unwrap_or(0);

    if files.is_empty() {
        return String::from("No items");
    }

    let selected = path.join(&files[i]);

    if selected.is_dir() {
        list(&selected, show_hidden).join("\n") // ← fixed, show SELECTED dir
    //           ↑
    //       now shows contents of selected folder
    } else {
        fs::read_to_string(&selected).unwrap_or(String::from("[Binary file]"))
    }
}

