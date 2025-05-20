use std::collections::HashMap;

use macroquad::input::KeyCode;

use super::Command;

pub fn default_keymap() -> HashMap<KeyCode, Command> {
    let mut keys = HashMap::new();
    keys.insert(
        KeyCode::Kp8,
        Command::Move {
            pos: (0, -1).into(),
        },
    );
    keys.insert(
        KeyCode::Kp7,
        Command::Move {
            pos: (-1, -1).into(),
        },
    );
    keys.insert(
        KeyCode::Kp4,
        Command::Move {
            pos: (-1, 0).into(),
        },
    );
    keys.insert(
        KeyCode::Kp1,
        Command::Move {
            pos: (-1, 1).into(),
        },
    );
    keys.insert(KeyCode::Kp2, Command::Move { pos: (0, 1).into() });
    keys.insert(KeyCode::Kp3, Command::Move { pos: (1, 1).into() });
    keys.insert(KeyCode::Kp6, Command::Move { pos: (1, 0).into() });
    keys.insert(
        KeyCode::Kp9,
        Command::Move {
            pos: (1, -1).into(),
        },
    );

    keys.insert(KeyCode::D, Command::Drop);
    keys.insert(KeyCode::I, Command::Inventory);
    keys.insert(KeyCode::R, Command::Unequip);
    keys.insert(KeyCode::Semicolon, Command::Look);
    keys.insert(KeyCode::Kp5, Command::Wait);
    keys.insert(KeyCode::F, Command::Fire);
    keys.insert(KeyCode::Period, Command::GoDownStairs);
    keys.insert(KeyCode::G, Command::Pickup);
    keys.insert(KeyCode::Apostrophe, Command::DevConsole);

    keys
}
