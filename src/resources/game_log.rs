use bevy::prelude::*;

#[derive(Resource)]
pub struct GameLog {
    pub logs: Vec<String>,
    pub is_dirty: bool,
}

impl GameLog {
    pub fn new() -> Self {
        Self { logs: Vec::new(), is_dirty: true }
    }

    pub fn append_log(&mut self, log: String) {
        self.logs.push(log);
    }
}
