use serde::Deserialize;

use super::Consumable;

#[derive(Debug, Deserialize)]
pub struct ScriptCommands {
    commands: Command,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub name: String,
    pub target: String,
    pub consumable: Consumable,
}
