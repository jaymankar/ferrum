use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

// A simple alias to make the code more readable
pub type KeyMap = HashMap<KeyEvent, Action>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveUp,
    MoveDown,
    Quit,
    MoveBack,
    MoveForward,
    Enter,
    Backspace,
    Rename,
    Delete,
    CreateFile,
    CreateDir,
    HiddenFile,
    Copy,
    Cut,
    Paste,
}

pub fn default_keymap() -> KeyMap {
    let mut map = HashMap::new();

    // Helper to create a KeyEvent with no modifiers
    let key = |code: KeyCode| KeyEvent::new(code, KeyModifiers::empty());

    map.insert(key(KeyCode::Up), Action::MoveUp);
    map.insert(key(KeyCode::Down), Action::MoveDown);
    map.insert(key(KeyCode::Left), Action::MoveBack);
    map.insert(key(KeyCode::Right), Action::MoveForward);
    map.insert(key(KeyCode::Enter), Action::Enter);
    map.insert(key(KeyCode::Backspace), Action::Backspace);

    // Vim-style
    map.insert(key(KeyCode::Char('k')), Action::MoveUp);
    map.insert(key(KeyCode::Char('j')), Action::MoveDown);
    map.insert(key(KeyCode::Char('h')), Action::MoveBack);
    map.insert(key(KeyCode::Char('l')), Action::MoveForward);

    // Actions
    map.insert(key(KeyCode::Char('d')), Action::Delete);
    map.insert(key(KeyCode::Char('r')), Action::Rename);
    map.insert(key(KeyCode::Char('f')), Action::CreateFile);
    map.insert(key(KeyCode::Char('n')), Action::CreateDir);
    map.insert(key(KeyCode::Char('.')), Action::HiddenFile);
    map.insert(key(KeyCode::Char('q')), Action::Quit);
    map.insert(key(KeyCode::Char('y')), Action::Copy);
    map.insert(key(KeyCode::Char('x')), Action::Cut);
    map.insert(key(KeyCode::Char('p')), Action::Paste);

    // Example of a modifier: Ctrl + c to quit
    map.insert(
        KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        Action::Quit,
    );

    map // Return the map
}
