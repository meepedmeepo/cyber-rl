mod commands;
pub use commands::*;

use macroquad::input::KeyCode;
use std::collections::HashMap;

pub struct InputManager {
    //map of currently held down keys and time in seconds they have been down for
    inputs_held: HashMap<KeyCode, f32>,
}

impl InputManager {
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
    }
}

pub fn get_input() {
    todo!()
}
