use crate::gamelog::{DebugLog, DEBUGLOG};
mod parser;

struct Console {
    buffer: &'static DebugLog,
    current_command: String,
    is_visible: bool,
}

impl Console {
    pub fn new() -> Self {
        Self {
            buffer: &DEBUGLOG,
            current_command: String::new(),
            is_visible: false,
        }
    }

    ///ran when enter key is pressed and terminal is in focus
    pub fn run_cmd(&mut self) {
        self.buffer.add_log(self.current_command.clone());
        self.current_command.clear();
    }
}
