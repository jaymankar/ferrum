use std::{
    fs::{self},
    io,
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

use crate::app::state::{FileEntry, SortMode};

pub fn list(path: &PathBuf, show_hidden: bool) -> io::Result<Vec<FileEntry>> {
    let entries = fs::read_dir(path)?;

    let mut result = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // hidden filter
        if !show_hidden && name.starts_with('.') {
            continue;
        }

        let (is_dir, size, modify) = match entry.metadata() {
            Ok(meta) => (
                meta.is_dir(),
                meta.size(),
                meta.modified()
                    .unwrap_or_else(|_| std::time::SystemTime::now()),
            ),
            Err(_) => {
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                (is_dir, 0u64, std::time::SystemTime::now())
            }
        };
        result.push(FileEntry {
            name,
            path,
            is_dir,
            size,
            modify,
        });
    }

    Ok(result)
}

pub fn delete(file: &FileEntry) -> io::Result<()> {
    if file.path.exists() {
        trash::delete(&file.path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    Ok(())
}

pub fn rename(file: &FileEntry, new_name: &str) -> io::Result<()> {
    let new_path = file.path.parent().unwrap().join(new_name);

    fs::rename(&file.path, new_path)?;
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

/// Preview selected file or directory
pub fn preview(file: &FileEntry, show_hidden: bool) -> String {
    if file.is_dir {
        match fs::read_dir(&file.path) {
            Ok(entries) => {
                let mut out = Vec::new();

                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();

                    if !show_hidden && name.starts_with('.') {
                        continue;
                    }

                    out.push(name);
                }

                out.join("\n")
            }
            Err(e) => format!("[Error reading directory: {}]", e),
        }
    } else {
        fs::read_to_string(&file.path).unwrap_or_else(|_| String::from("[Binary file]"))
    }
}
pub fn go_to_parent(path: &PathBuf) -> PathBuf {
    path.parent().unwrap_or(path).to_path_buf()
}

// Copy , Cut , Paste

pub fn copy_item(src: &PathBuf, dst_dir: &PathBuf) -> io::Result<()> {
    let name = src
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "source has no filename"))?;
    let dst = dst_dir.join(name);

    if src.is_dir() {
        copy_dir(src, &dst)?;
    } else {
        fs::copy(src, &dst)?;
    }
    Ok(())
}

pub fn move_item(src: &PathBuf, dst_dir: &PathBuf) -> io::Result<()> {
    let name = src
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "source has no filename"))?;
    let dst = dst_dir.join(name);

    if fs::rename(src, &dst).is_err() {
        if src.is_dir() {
            copy_dir(src, &dst)?;
        } else {
            fs::copy(src, &dst);
        }
        trash::delete(src).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    Ok(())
}

pub fn copy_dir(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)?.flatten() {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn sort(files: &mut Vec<FileEntry>, mode: &SortMode) {
    match mode {
        SortMode::FileName => {
            files.sort_by(|a, b| {
                b.is_dir
                    .cmp(&a.is_dir)
                    .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            });
        }
        SortMode::Size => {
            files.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(b.size.cmp(&a.size)));
        }
        SortMode::Date => {
            files.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(b.modify.cmp(&a.modify)));
        }
    }
}
