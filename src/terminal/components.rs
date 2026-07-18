use bevy::prelude::*;

#[derive(Component)]
pub struct TerminalHistory {
    pub lines: Vec<String>,
    pub scroll: usize,
}

impl Default for TerminalHistory {
    fn default() -> Self {
        Self {
            lines: vec![
                "FLOCK TERM v0.1.0".into(),
                "Type 'help' for commands.".into(),
            ],
            scroll: 0,
        }
    }
}

#[derive(Component, Default)]
pub struct TerminalInput {
    pub buffer: String,
}

#[derive(Resource, Default)]
pub struct CommandQueue {
    pub pending: Vec<String>,
}

#[derive(Message)]
pub struct PrintToTerminal(pub String);
