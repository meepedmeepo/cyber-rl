use crate::{
    effects::{add_effect, EffectType, Targets},
    logging::gamelog::DEBUGLOG,
    scripting::ScriptingEngine,
};

use super::{Command, ScriptCommands};
use rhai::ImmutableString;
use serde::Deserialize;
use std::{fs, sync::LazyLock};

static COMMANDS: LazyLock<ScriptingCommandLoader> = LazyLock::new(ScriptingCommandLoader::new);

#[derive(Debug, Deserialize)]
pub struct ScriptingCommandLoader {
    script_commands: ScriptCommands,
}

impl ScriptingCommandLoader {
    pub fn new() -> Self {
        let data = fs::read_to_string(std::path::Path::new("./src/raws/commands.json"))
            .expect("Unable to read commands.json");
        //println!("{}", data);
        let decoder: ScriptingCommandLoader =
            serde_json::from_str(&data).expect("Unable to parse JSON");

        decoder
    }

    pub fn register_commands(&self, engine: &mut ScriptingEngine) {
        let handler = engine.get_engine_mut();
        handler.register_fn("invoke", invoke_command);
    }

    pub fn find_command(&self, name: &str) -> Option<Command> {
        self.script_commands
            .commands
            .iter()
            .filter(|cmd| if cmd.name == name { true } else { false })
            .next()
            .cloned()
    }
}

pub fn load_scripting_commands(engine: &mut ScriptingEngine) {
    COMMANDS.register_commands(engine);
    DEBUGLOG.add_log(String::from("Scripting commands loaded."));
}

fn invoke_command(command: ImmutableString) {
    match COMMANDS.find_command(&command) {
        None => DEBUGLOG.add_log(format!("Error: No such command as {}", command)),

        Some(command) => add_effect(
            None,
            EffectType::ConsoleCommand { command },
            Targets::Tile { tile_idx: 0 }, //this isn't actually used for ConsoleCommand effects currently so is placeholder
        ),
    }
}
