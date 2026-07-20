use super::components::*;

use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
        mouse::MouseWheel,
    },
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};

const VISIBLE_LINES: usize = 17;
const MAX_HISTORY: usize = 1000;

pub fn setup_terminal_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/default.ttf");

    commands.spawn((
        Text2d::new(format!(
            "Flock Monitoring System v{}\nRoom Measures:\n",
            env!("CARGO_PKG_VERSION")
        )),
        TextFont {
            font: font.clone().into(),
            font_size: 16.0.into(),
            ..default()
        },
        TextLayout::justify(Justify::Start),
        Anchor::TOP_LEFT,
        TextColor(Color::srgb(0.9, 0.5, 0.1)),
        Transform::from_xyz(-620.0, 340.0, 10.0),
    ));

    let window_size = Vec2::new(600.0, 420.0);
    let min_size = Vec2::new(600.0, 40.0);

    let parent_window = commands
        .spawn((
            TerminalWindow {
                is_dragging: false,
                drag_offset: Vec2::ZERO,
                is_minimized: false,
                full_size: window_size,
                min_size,
            },
            Sprite {
                color: Color::srgba(0.05, 0.05, 0.05, 0.8),
                custom_size: Some(window_size),
                ..default()
            },
            Transform::from_xyz(300.0, 110.0, 100.0),
            Anchor::TOP_CENTER,
        ))
        .id();

    let title_bar = commands
        .spawn((
            Text2d::new(format!("[-] FLOCK TERM v{}", env!("CARGO_PKG_VERSION"))),
            TextFont {
                font: font.clone().into(),
                font_size: 14.0.into(),
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.7, 0.2)),
            Transform::from_xyz(-285.0, -15.0, 1.0),
            Anchor::TOP_LEFT,
        ))
        .id();

    let content_group = commands
        .spawn((TerminalContent, Transform::default(), Visibility::Inherited))
        .id();

    let history = commands
        .spawn((
            TerminalHistory::default(),
            Text2d::new("Type 'help' for commands.\n"),
            TextFont {
                font: font.clone().into(),
                font_size: 16.0.into(),
                ..default()
            },
            TextLayout::justify(Justify::Start),
            Anchor::TOP_LEFT,
            TextColor(Color::srgb(0.4, 0.9, 0.4)),
            Transform::from_xyz(-285.0, -50.0, 1.0),
        ))
        .id();

    let mut cursor_entity = Entity::PLACEHOLDER;
    let mut left_text_entity = Entity::PLACEHOLDER;
    let mut cursor_char_entity = Entity::PLACEHOLDER;
    let mut right_text_entity = Entity::PLACEHOLDER;

    let input = commands
        .spawn((
            TerminalInput::default(),
            Transform::from_xyz(-285.0, -390.0, 1.0),
            Visibility::Inherited,
        ))
        .with_children(|parent| {
            cursor_entity = parent
                .spawn((
                    Sprite {
                        color: Color::srgba(0.2, 0.9, 0.2, 0.8),
                        custom_size: Some(Vec2::new(10.8, 22.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 3.0, 0.5),
                    Anchor::TOP_LEFT,
                ))
                .id();

            left_text_entity = parent
                .spawn((
                    Text2d::new("> "),
                    TextFont {
                        font: font.clone().into(),
                        font_size: 18.0.into(),
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.8, 0.2)),
                    Anchor::TOP_LEFT,
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ))
                .id();

            cursor_char_entity = parent
                .spawn((
                    Text2d::new(""),
                    TextFont {
                        font: font.clone().into(),
                        font_size: 18.0.into(),
                        ..default()
                    },
                    TextColor(Color::srgb(0.05, 0.08, 0.05)),
                    Anchor::TOP_LEFT,
                    Transform::from_xyz(0.0, 0.0, 1.2),
                ))
                .id();

            right_text_entity = parent
                .spawn((
                    Text2d::new(""),
                    TextFont {
                        font: font.into(),
                        font_size: 18.0.into(),
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.8, 0.2)),
                    Anchor::TOP_LEFT,
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ))
                .id();
        })
        .insert(TerminalInputUI {
            cursor_entity,
            left_text_entity,
            cursor_char_entity,
            right_text_entity,
        })
        .id();

    commands
        .entity(content_group)
        .add_children(&[history, input]);
    commands
        .entity(parent_window)
        .add_children(&[title_bar, content_group]);
}

pub fn handle_window_drag(
    window: Single<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut terminal_query: Query<(&mut TerminalWindow, &mut Transform, &mut Sprite, &Children)>,
    mut content_query: Query<&mut Visibility, With<TerminalContent>>,
    mut text_query: Query<&mut Text2d>,
) {
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let world_x = cursor_pos.x - window.width() / 2.0;
    let world_y = -(cursor_pos.y - window.height() / 2.0);
    let mouse_pos = Vec2::new(world_x, world_y);

    for (mut term, mut transform, mut sprite, children) in terminal_query.iter_mut() {
        let pos = transform.translation.truncate();
        let half_width = term.full_size.x / 2.0;

        let title_bar_rect = Rect::from_center_size(
            Vec2::new(pos.x, pos.y - 15.0),
            Vec2::new(term.full_size.x, 30.0),
        );

        let minimize_rect = Rect::from_center_size(
            Vec2::new(pos.x - half_width + 25.0, pos.y - 15.0),
            Vec2::new(50.0, 30.0),
        );

        if buttons.just_pressed(MouseButton::Left) {
            if minimize_rect.contains(mouse_pos) {
                term.is_minimized = !term.is_minimized;

                if term.is_minimized {
                    sprite.custom_size = Some(term.min_size);

                    for child in children.iter() {
                        if let Ok(mut vis) = content_query.get_mut(child) {
                            *vis = Visibility::Hidden;
                        }
                        if let Ok(mut text) = text_query.get_mut(child)
                            && text.0.starts_with("[-]")
                        {
                            text.0 = text.0.replace("[-]", "[+]");
                        }
                    }
                } else {
                    sprite.custom_size = Some(term.full_size);

                    for child in children.iter() {
                        if let Ok(mut vis) = content_query.get_mut(child) {
                            *vis = Visibility::Inherited;
                        }
                        if let Ok(mut text) = text_query.get_mut(child)
                            && text.0.starts_with("[+]")
                        {
                            text.0 = text.0.replace("[+]", "[-]");
                        }
                    }
                }
            } else if title_bar_rect.contains(mouse_pos) {
                term.is_dragging = true;
                term.drag_offset = pos - mouse_pos;
            }
        }

        if buttons.just_released(MouseButton::Left) {
            term.is_dragging = false;
        }

        if term.is_dragging {
            transform.translation.x = mouse_pos.x + term.drag_offset.x;
            transform.translation.y = mouse_pos.y + term.drag_offset.y;
        }
    }
}

pub fn update_terminal_history(
    mut messages: MessageReader<PrintToTerminal>,
    mut terminal: Single<(&mut TerminalHistory, &mut Text2d)>,
) {
    let (history, text) = &mut *terminal;

    let mut changed = false;

    for msg in messages.read() {
        for line in msg.0.lines() {
            history.lines.push(line.to_string());
            history.scroll = 0;
            changed = true;
        }
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut input: Single<&mut TerminalInput>,
    mut command_queue: ResMut<CommandQueue>,
    mut print_events: MessageWriter<PrintToTerminal>,
) {
    let ctrl_pressed = keyboard_input.pressed(KeyCode::ControlLeft)
        || keyboard_input.pressed(KeyCode::ControlRight);

    for event in key_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        if ctrl_pressed {
            match &event.logical_key {
                Key::Character(c) if c.eq_ignore_ascii_case("c") => {
                    input.buffer.clear();
                    input.cursor_pos = 0;
                    continue;
                }
                Key::Character(c) if c.eq_ignore_ascii_case("u") => {
                    let cursor_pos = input.cursor_pos;
                    input.buffer.drain(..cursor_pos);
                    input.cursor_pos = 0;
                    continue;
                }
                _ => {}
            }
        }

        match &event.logical_key {
            Key::Enter => {
                let cmd: String = input.buffer.iter().collect();
                let cmd = cmd.trim().to_string();

                if !cmd.is_empty() {
                    input.history.push(cmd.clone());
                    input.history_index = input.history.len();
                    command_queue.pending.push(cmd.clone());
                }

                input.buffer.clear();
                input.cursor_pos = 0;
            }
            Key::Backspace => {
                if input.cursor_pos > 0 {
                    input.cursor_pos -= 1;
                    let cursor_pos = input.cursor_pos;
                    input.buffer.remove(cursor_pos);
                }
            }
            Key::Delete => {
                if input.cursor_pos < input.buffer.len() {
                    let cursor_pos = input.cursor_pos;
                    input.buffer.remove(cursor_pos);
                }
            }
            Key::ArrowLeft => {
                if input.cursor_pos > 0 {
                    input.cursor_pos -= 1;
                }
            }
            Key::ArrowRight => {
                if input.cursor_pos < input.buffer.len() {
                    input.cursor_pos += 1;
                }
            }
            Key::Home => {
                input.cursor_pos = 0;
            }
            Key::End => {
                input.cursor_pos = input.buffer.len();
            }
            Key::ArrowUp => {
                if input.history_index > 0 {
                    input.history_index -= 1;
                    input.buffer = input.history[input.history_index].chars().collect();
                    input.cursor_pos = input.buffer.len();
                }
            }
            Key::ArrowDown => {
                if input.history_index + 1 < input.history.len() {
                    input.history_index += 1;
                    input.buffer = input.history[input.history_index].chars().collect();
                    input.cursor_pos = input.buffer.len();
                } else if input.history_index < input.history.len() {
                    input.history_index = input.history.len();
                    input.buffer.clear();
                    input.cursor_pos = 0;
                }
            }
            Key::Space => {
                let cursor_pos = input.cursor_pos;
                input.buffer.insert(cursor_pos, ' ');
                input.cursor_pos += 1;
            }
            Key::Character(str) => {
                for c in str.chars() {
                    if !c.is_control() {
                        let cursor_pos = input.cursor_pos;
                        input.buffer.insert(cursor_pos, c);
                        input.cursor_pos += 1;
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn update_terminal_display(
    input_query: Query<(&TerminalInput, &TerminalInputUI)>,
    mut text_q: Query<&mut Text2d>,
    mut transform_q: Query<&mut Transform>,
) {
    const CHAR_WIDTH: f32 = 10.8;

    for (input, ui) in &input_query {
        let prompt_and_left: String = std::iter::once('>')
            .chain(std::iter::once(' '))
            .chain(input.buffer[..input.cursor_pos].iter().copied())
            .collect();

        let (cursor_char_str, right_str) = if input.cursor_pos < input.buffer.len() {
            let c = input.buffer[input.cursor_pos];
            let rest: String = input.buffer[input.cursor_pos + 1..].iter().collect();
            (c.to_string(), rest)
        } else {
            (" ".to_string(), "".to_string())
        };

        let left_width = prompt_and_left.chars().count() as f32 * CHAR_WIDTH;
        let char_width_offset = left_width + CHAR_WIDTH;

        if let Ok(mut text) = text_q.get_mut(ui.left_text_entity) {
            text.0 = prompt_and_left;
        }
        if let Ok(mut text) = text_q.get_mut(ui.cursor_char_entity) {
            text.0 = cursor_char_str;
        }
        if let Ok(mut text) = text_q.get_mut(ui.right_text_entity) {
            text.0 = right_str;
        }

        if let Ok(mut transform) = transform_q.get_mut(ui.cursor_char_entity) {
            transform.translation.x = left_width;
        }
        if let Ok(mut transform) = transform_q.get_mut(ui.right_text_entity) {
            transform.translation.x = char_width_offset;
        }
        if let Ok(mut transform) = transform_q.get_mut(ui.cursor_entity) {
            transform.translation.x = left_width;
        }
    }
}
