use std::sync::{LazyLock, Mutex};

pub static DEBUGLOG: DebugLog = DebugLog::new();

pub struct DebugLog {
    log: LazyLock<Mutex<GameLog>>,
}

impl DebugLog {
    pub const fn new() -> Self {
        DebugLog {
            log: LazyLock::new(|| Mutex::new(GameLog::new())),
        }
    }

    pub fn add_log(&self, msg: String) {
        self.log.lock().unwrap().add_log(msg.clone());
        println!("debug: {}", msg);
    }
}

#[derive(Debug, Clone)]
pub struct GameLog {
    pub entries: Vec<String>,
    pub index: usize,
}

impl GameLog {
    pub fn new() -> GameLog {
        GameLog {
            entries: Vec::new(),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn add_log(&mut self, msg: String) {
        self.entries.push(". ".to_string() + &msg);
        //self.index+=1;
    }

    pub fn view_log(&self, num_entries: usize) -> Vec<String> {
        self.entries
            .iter()
            .rev()
            .skip(self.index)
            .take(num_entries)
            .map(|s| s.clone())
            .collect::<Vec<String>>()
    }
}

impl Iterator for GameLog {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.iter().next().cloned()
    }
}
