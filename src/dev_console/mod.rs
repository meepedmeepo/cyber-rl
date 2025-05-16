use std::{
    cell::{Cell, LazyCell},
    sync::LazyLock,
};

use crate::{
    gamelog::{DebugLog, DEBUGLOG},
    scripting,
};
//pub static TERMINAL: LazyLock<Terminal> = LazyLock::new(Terminal::new());
pub struct Terminal {
    buffer: &'static DebugLog,
    current_command: String,
    is_visible: bool,
    engine: scripting::ScriptingEngine,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            buffer: &DEBUGLOG,
            current_command: String::new(),
            is_visible: false,
            engine: scripting::init_engine(),
        }
    }

    pub fn set_cmd(&mut self, cmd: String) {
        self.current_command = cmd;
    }

    ///Runs command on the embedded rhai scripting engine
    pub fn run_cmd(&mut self) {
        self.buffer.add_log(self.current_command.clone());
        self.engine.run_command(self.current_command.clone());
        self.current_command.clear();
    }
}
