use crate::terminal::components::{Measures, PrintToMeasures, PrintToTerminal, TerminalHistory};

use super::components::{CommandQueue, TerminalInput};
use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};

pub const MAX_TERM_HISTORY: usize = 50;

pub fn setup_terminal_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.9)),
        ))
        .with_child((
            Measures,
            Text::new("Room Measures:\n"),
            TextFont {
                font: asset_server.load("fonts/default.ttf").into(),
                font_size: 16.0.into(),
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 1.0)),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_grow: 1.0,
                    overflow: Overflow::clip_y(),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    min_height: Val::Px(0.0),
                    ..default()
                })
                .with_children(|history_container| {
                    history_container.spawn((
                        TerminalHistory,
                        Text::new("FLOCK TERM v0.1.0\nType 'help' for commands.\n"),
                        TextFont {
                            font: asset_server.load("fonts/default.ttf").into(),
                            font_size: 16.0.into(),
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
            parent.spawn((
                TerminalInput {
                    buffer: String::with_capacity(128),
                },
                Text::new("> "),
                TextFont {
                    font: asset_server.load("fonts/default.ttf").into(),
                    font_size: 18.0.into(),
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 0.2)),
                Node {
                    flex_shrink: 0.0,
                    ..default()
                },
            ));
        });
}

pub fn update_measures(
    mut messages: MessageReader<PrintToMeasures>,
    mut measures: Single<&mut Text, With<Measures>>,
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
    mut history_query: Single<&mut Text, With<TerminalHistory>>,
) {
    if messages.is_empty() {
        return;
    }

    for message in messages.read() {
        history_query.0.push_str(&message.0);
        history_query.0.push('\n');
    }

    let lines: Vec<&str> = history_query.0.lines().collect();
    if lines.len() > MAX_TERM_HISTORY {
        let truncated = lines[lines.len() - MAX_TERM_HISTORY..].join("\n");
        history_query.0 = truncated + "\n";
    }
}

pub fn handle_typing(
    mut key_events: MessageReader<KeyboardInput>,
    mut terminal: Single<(&mut TerminalInput, &mut Text)>,
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
