# ferrum 🦀

> A fast, lightweight TUI file manager built in Rust

<!-- Add demo GIF here -->
<!-- ![demo](demo.gif) -->

---

## Features

- Navigate directories with vim keybindings
- File preview — text, code, and directory contents
- Create, rename, and delete files and folders
- Hidden file toggle
- Parent directory panel
- Command mode
- Error handling with status bar
- Clean modular architecture

---

## Install

### Prerequisites

- Rust and Cargo installed — [rustup.rs](https://rustup.rs)
- Linux
- A [Nerd Font](https://www.nerdfonts.com) set in your terminal (for icons)

### Build from source

```bash
git clone https://github.com/jaymankar/ferrum.git
cd ferrum
cargo build --release
./target/release/ferrum
```

### Or run directly

```bash
cargo run
```

---

## Keybindings

| Key | Action |
|-----|--------|
| `↑` / `↓` | Move up / down |
| `→` | Enter folder |
| `←` | Go up one folder |
| `d` | Delete file or folder |
| `r` | Rename |
| `f` | New file |
| `n` | New folder |
| `.` | Toggle hidden files |
| `/` | Command mode |
| `q` | Quit |
| `y` | Copy |
| 'x' | Cut  |
| 'p' | paste |



---

## Project Structure

```
src/
├── main.rs
├── app/
│   ├── state.rs      — app state
│   ├── action.rs     — navigation logic
│   └── app.rs        — main loop
├── ui/
│   ├── draw.rs       — main render
│   ├── layout.rs     — layouts
│   └── components/
│       ├── file_list.rs
│       ├── preview.rs
│       ├── parent_dir.rs
│       └── status_bar.rs
├── fs/
│   ├── explorer.rs   — file operations
│   └── entry.rs      — file metadata
├── input/
│   └── handler.rs    — keyboard handling
└── config/
    └── config.rs     — settings
```

---

## Built With

- [Ratatui](https://ratatui.rs) — TUI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) — terminal backend
- [Trash](https://github.com/Byron/trash-rs) — safe delete
- [Color Eyre](https://github.com/eyre-rs/color-eyre) — error handling

---

## Roadmap

- [ ] Syntax highlighting in preview
- [ ] File size and date in list
- [ ] Bookmarks
- [ ] Search
- [ ] Config file for custom keybindings
- [ ] Image preview
- [ ] Git status indicators

---

## Author

**Jay Mankar** — self taught, 18 years old

Built from scratch with no tutorials. Just docs, papers, and a lot of `cargo check`.

---

## License

MIT
