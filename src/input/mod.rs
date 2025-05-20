mod commands;
mod input_system;
pub use commands::*;
pub use input_system::*;

use macroquad::input::KeyCode;
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::{Arc, LazyLock, Mutex, MutexGuard},
};

pub static INPUT: LazyLock<InputManagerLock> = LazyLock::new(InputManagerLock::new);

pub struct InputManagerLock(Arc<Mutex<InputManager>>);

impl InputManagerLock {
    fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(InputManager::new())),
        }
    }
    pub fn lock(&self) -> MutexGuard<InputManager> {
        self.0.lock().unwrap()
    }
}

pub struct InputManager {
    //map of currently held down keys and time in seconds they have been down for
    inputs_held: HashMap<KeyCode, f32>,
    commands: CommandManager,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            inputs_held: HashMap::new(),
            commands: CommandManager::new(),
        }
    }

    ///Updates list of currently held down keys and how long they have been held for
    pub fn tick(&mut self, delta_t: f32) {
        for (_key, time) in self.inputs_held.iter_mut() {
            *time += delta_t;
        }

        for key in macroquad::input::get_keys_pressed() {
            self.inputs_held.insert(key, 0f32);
        }

        for key in macroquad::input::get_keys_released() {
            self.inputs_held.remove(&key);
        }

        //updates command manager with inputs from mapping hashmap of keycodes of currently held down keys to a hashset
        self.commands.tick(
            delta_t,
            self.inputs_held
                .keys()
                .map(|k| *k)
                .collect::<HashSet<KeyCode>>(),
        );
    }

    pub fn get_command(&mut self) -> Option<Command> {
        self.commands.get_command()
    }
}
