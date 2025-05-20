use std::collections::{HashMap, HashSet, VecDeque};

use bracket_lib::prelude::Point;
use macroquad::input::KeyCode;

use crate::utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
//internal representation of actions to be taken by player, to decouple them from having fixed keys
pub enum Command {
    Move { pos: Point },
    Wait,
    GoDownStairs,
    Inventory,
    Pickup,
    Drop,
    Equip,
    Unequip,
    Fire,
    Look,
    Save,

    Quit,

    None,
}

pub struct CommandManager {
    command_queue: VecDeque<Command>,
    cooldown: utils::timer::Timer,
    command_locked: bool,
    keymap: HashMap<KeyCode, Command>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            command_queue: VecDeque::new(),
            cooldown: utils::timer::Timer::new_stopped(0.08),
            command_locked: false,
            keymap: HashMap::new(), //todo: actually add keybindings and keybinding loading
        }
    }

    pub fn tick(&mut self, delta_t: f32, inputs: HashSet<KeyCode>) {
        self.command_queue.clear();
        self.cooldown.tick(delta_t);
        if !self.command_locked {
            self.translate_inputs(inputs);
        }
    }

    pub fn translate_inputs(&mut self, inputs: HashSet<KeyCode>) {
        for key in inputs.iter() {
            match self.keymap.get(key) {
                None => {}
                Some(command) => {
                    self.command_queue.push_back(*command);
                }
            }
        }
    }

    pub fn get_command(&mut self) -> Option<Command> {
        if self.command_locked || !self.cooldown.timer_elapsed() {
            None
        } else {
            let cmd = self.command_queue.pop_front();

            if cmd.is_some() {
                self.cooldown.reset();
            }

            cmd
        }
    }

    pub fn disable(&mut self) {
        self.command_locked = true;
    }

    pub fn enable(&mut self) {
        self.command_locked = false;
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        CommandManager {
            command_queue: VecDeque::new(),
            cooldown: utils::timer::Timer::new(0.08),
            command_locked: false,
            keymap: HashMap::new(),
        }
    }
}
