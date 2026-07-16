use bevy::prelude::*;

#[derive(Component)]
pub struct TerminalRoot;

#[derive(Component)]
pub struct TerminalHistory;

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

#[derive(Component)]
pub struct Measures;

#[derive(Message)]
pub struct PrintToMeasures(pub String);
