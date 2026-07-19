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
                color: Color::srgba(0.05, 0.08, 0.05, 1.0),
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

    let input = commands
        .spawn((
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
            Transform::from_xyz(-285.0, -390.0, 1.0),
        ))
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
