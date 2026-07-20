use bevy::prelude::*;

#[derive(Component)]
pub struct TerminalWindow {
    pub is_dragging: bool,
    pub drag_offset: Vec2,
    pub is_minimized: bool,
    pub full_size: Vec2,
    pub min_size: Vec2,
}

#[derive(Component)]
pub struct TerminalContent;

#[derive(Component)]
pub struct TerminalHistory {
    pub lines: Vec<String>,
    pub scroll: usize,
}

impl Default for TerminalHistory {
    fn default() -> Self {
        Self {
            lines: vec!["Type 'help' for commands.".into()],
            scroll: 0,
        }
    }
}

#[derive(Component, Default)]
pub struct TerminalInput {
    pub buffer: Vec<char>,
    pub cursor_pos: usize,
    pub history: Vec<String>,
    pub history_index: usize,
}

#[derive(Component)]
pub struct TerminalInputUI {
    pub cursor_entity: Entity,
    pub left_text_entity: Entity,
    pub cursor_char_entity: Entity,
    pub right_text_entity: Entity,
}

#[derive(Component)]
pub struct TerminalLeftText;

#[derive(Component)]
pub struct TerminalCursorCharText;

#[derive(Component)]
pub struct TerminalRightText;

#[derive(Resource, Default)]
pub struct CommandQueue {
    pub pending: Vec<String>,
}

#[derive(Message)]
pub struct PrintToTerminal(pub String);
