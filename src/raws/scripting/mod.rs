mod loader;

use super::Consumable;
pub use loader::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScriptCommands {
    commands: Vec<Command>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Command {
    pub name: String,
    pub target: String,
    pub consumable: Consumable,
}

impl Command {
    pub fn parse_command(&self) -> impl Fn() {
        move || print!("test")
    }
}
