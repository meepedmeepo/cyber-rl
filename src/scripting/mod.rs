use std::sync::Mutex;

use crate::gamelog::DEBUGLOG;

pub struct ScriptingEngine {
    engine: rhai::Engine,
    pub scope: Mutex<rhai::Scope<'static>>,
}

impl ScriptingEngine {
    pub fn new() -> Self {
        Self {
            engine: rhai::Engine::new(),
            scope: Mutex::new(rhai::Scope::new()),
        }
    }
    pub fn run_command(&self, input: String) {
        let res = self
            .engine
            .run_with_scope(&mut self.scope.lock().unwrap(), &input);
    }

    pub fn get_engine(&self) -> &rhai::Engine {
        &self.engine
    }

    pub fn get_engine_mut(&mut self) -> &mut rhai::Engine {
        &mut self.engine
    }
}

///Instantiates and initialises basic configuration of the rhai scripting engine to be used for dev console
pub fn init_engine() -> ScriptingEngine {
    let mut handler = ScriptingEngine::new();

    //redirects debug and print messages from the scripting engine to the debug log
    handler
        .get_engine_mut()
        .on_print(|msg| DEBUGLOG.add_log(msg.to_string()))
        .on_debug(|x, src, pos| {
            let src = src.unwrap_or("unknown");
            let msg = format!("DEBUG of {} at {:?}: {}", src, pos, x);

            DEBUGLOG.add_log(msg);
        });

    handler
}
