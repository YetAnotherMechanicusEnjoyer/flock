use crate::terminal::components::{Measures, PrintToMeasures, PrintToTerminal, TerminalHistory};

use super::components::{CommandQueue, TerminalInput};
use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
        mouse::MouseWheel,
    },
    prelude::*,
    sprite::Anchor,
};

const VISIBLE_LINES: usize = 15;
const MAX_HISTORY: usize = 1000;

pub fn setup_terminal_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/default.ttf");

    commands.spawn((
        Text2d::new("Flock Monitoring System v0.1.0\nRoom Measures:\n"),
        TextFont {
            font: font.clone().into(),
            font_size: 16.0.into(),
            ..default()
        },
        TextLayout::justify(Justify::Start),
        Anchor::TOP_LEFT,
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Transform::from_xyz(-620.0, 340.0, 100.0),
    ));

    commands.spawn((
        TerminalHistory::default(),
        Text2d::new("FLOCK TERM v0.1.0\nType 'help' for commands.\n"),
        TextFont {
            font: font.clone().into(),
            font_size: 16.0.into(),
            ..default()
        },
        TextLayout::justify(Justify::Start),
        Anchor::TOP_LEFT,
        TextColor(Color::srgb(0.4, 0.9, 0.4)),
        Transform::from_xyz(-620.0, -40.0, 100.0),
    ));

    commands.spawn((
        TerminalInput {
            buffer: String::with_capacity(128),
        },
        Text2d::new("> "),
        TextFont {
            font: font.into(),
            font_size: 18.0.into(),
            ..default()
        },
        TextLayout::justify(Justify::Start),
        Anchor::TOP_LEFT,
        TextColor(Color::srgb(0.2, 0.8, 0.2)),
        Transform::from_xyz(-620.0, -330.0, 100.0),
    ));
}

pub fn update_measures(
    mut messages: MessageReader<PrintToMeasures>,
    mut measures: Single<&mut Text2d, With<Measures>>,
) {
    if messages.is_empty() {
        return;
    }

    measures.clear();

    for message in messages.read() {
        measures.0.push_str(&message.0);
        measures.0.push('\n');
    }

    let len = measures.0.len();
    measures.0.truncate(len.saturating_sub(1));
}

pub fn update_terminal_history(
    mut messages: MessageReader<PrintToTerminal>,
    mut terminal: Single<(&mut TerminalHistory, &mut Text2d)>,
) {
    let (history, text) = &mut *terminal;

    let mut changed = false;

    for msg in messages.read() {
        history.lines.push(msg.0.clone());
        history.scroll = 0;
        changed = true;
    }

    if history.lines.len() > MAX_HISTORY {
        let excess = history.lines.len() - MAX_HISTORY;
        history.lines.drain(..excess);
    }

    if changed {
        rebuild_terminal(history, text);
    }
}

fn rebuild_terminal(history: &TerminalHistory, text: &mut Text2d) {
    text.clear();

    let len = history.lines.len();

    let end = len.saturating_sub(history.scroll);

    let start = end.saturating_sub(VISIBLE_LINES);

    for line in &history.lines[start..end] {
        text.push_str(line);
        text.push('\n');
    }
}

pub fn terminal_scroll(
    mut wheel: MessageReader<MouseWheel>,
    mut terminal: Single<(&mut TerminalHistory, &mut Text2d)>,
) {
    let (history, text) = &mut *terminal;

    let max_scroll = history.lines.len().saturating_sub(VISIBLE_LINES);

    let mut changed = false;

    for ev in wheel.read() {
        if ev.y > 0.0 {
            history.scroll = (history.scroll + 1).min(max_scroll);
            changed = true;
        } else if ev.y < 0.0 {
            history.scroll = history.scroll.saturating_sub(1);
            changed = true;
        }
    }

    if changed {
        rebuild_terminal(history, text);
    }
}

pub fn handle_typing(
    mut key_events: MessageReader<KeyboardInput>,
    mut terminal: Single<(&mut TerminalInput, &mut Text2d)>,
    mut command_queue: ResMut<CommandQueue>,
) {
    let (ref mut term_input, ref mut text) = *terminal;
    let mut changed = false;

    for event in key_events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                if !term_input.buffer.is_empty() {
                    command_queue.pending.push(term_input.buffer.clone());

                    term_input.buffer.clear();
                    changed = true;
                }
            }
            Key::Backspace => {
                if term_input.buffer.pop().is_some() {
                    changed = true;
                }
            }
            Key::Character(char_str) => {
                if !char_str.chars().any(|c| c.is_control()) {
                    term_input.buffer.push_str(char_str.as_str());
                    changed = true;
                }
            }
            Key::Space => {
                term_input.buffer.push(' ');
                changed = true;
            }
            _ => {}
        }
    }

    if changed {
        text.0 = format!("> {}", term_input.buffer);
    }
}
