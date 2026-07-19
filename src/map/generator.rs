use bevy::prelude::*;
use rand::{RngExt, rngs::ThreadRng, seq::SliceRandom};

use super::components::*;
use crate::{simulation::components::*, utils::consts::*};

pub fn spawn_procedural_ship(mut commands: Commands, config: Res<MapGenConfig>) {
    let mut rng = rand::rng();

    let initial_rect =
        Rect::from_center_size(Vec2::ZERO, Vec2::new(SHIP_SIZE_WIDTH, SHIP_SIZE_HEIGHT));
    let mut room_rects = Vec::new();
    split_rect(initial_rect, 4, &mut rng, &mut room_rects);

    let mut room_entities = Vec::new();
    let mut available_rooms = Vec::new();

    for rect in &room_rects {
        let center = rect.center();
        let width = rect.width();
        let height = rect.height();

        let ns = if center.y > 0.0 { "N" } else { "S" };
        let ew = if center.x > 0.0 { "E" } else { "W" };
        let room_id = format!("{}{}-{:02}", ns, ew, rng.random_range(10..99));

        let room_ent = commands
            .spawn((
                Room {
                    name: room_id.clone(),
                },
                Transform::from_xyz(center.x, center.y, ROOM_Z),
                RoomLayout { width, height },
                Oxygen(OXYGEN_MAX),
                OxygenDelta(0.0),
                HullIntegrity(100.0),
                RoomVisual,
                Sprite {
                    color: config.room_color.with_alpha(0.15),
                    custom_size: Some(Vec2::new(width - 2.0, height - 2.0)),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    RoomLabel,
                    Text2d::new(room_id.clone()),
                    TextFont {
                        font_size: 12.0.into(),
                        ..default()
                    },
                    TextColor(config.room_color),
                    Transform::from_xyz(0.0, height / 2.0 - 10.0, LABEL_Z - ROOM_Z),
                ));
            })
            .id();

        room_entities.push((room_ent, *rect));
        available_rooms.push((room_ent, room_id, *rect));
    }

    distribute_machines(&mut commands, &mut available_rooms, &mut rng);
    spawn_doors_on_edges(&mut commands, &room_entities);
}

fn distribute_machines(
    commands: &mut Commands,
    available_rooms: &mut Vec<(Entity, String, Rect)>,
    rng: &mut ThreadRng,
) {
    available_rooms.shuffle(rng);

    let machines_to_spawn = vec![
        MachineType::Reactor,
        MachineType::LifeSupport,
        MachineType::Server,
        MachineType::Server,
        MachineType::Cooler,
        MachineType::Cooler,
        MachineType::Cooler,
    ];

    for m_type in machines_to_spawn {
        if let Some((room_ent, room_id, rect)) = available_rooms.pop() {
            let m_id = format!("{}-{}", room_id, m_type.short_code());
            let center = rect.center();

            let size = Vec2::splat(crate::utils::consts::MACHINE_RENDER_SIZE);
            let custom_size = match m_type {
                MachineType::Server => Vec2::new(size.x / 2.0, size.y),
                MachineType::Reactor => size * 1.2,
                _ => size,
            };

            let color = Color::srgb(0.2, 0.8, 0.2);

            let mut machine_cmd = commands.spawn((
                Machine {
                    id_name: m_id,
                    machine_type: m_type,
                },
                Transform::from_xyz(center.x, center.y, MACHINE_Z),
                LocatedIn(room_ent),
                Temperature {
                    current: DEFAULT_TEMP,
                    target: DEFAULT_TEMP,
                },
                ThermalDelta(0.0),
                PowerState::Active,
                MachineVisual,
                Sprite {
                    color,
                    custom_size: Some(custom_size),
                    ..default()
                },
            ));

            machine_cmd.with_children(|parent| {
                parent.spawn((
                    MachineLabel,
                    Text2d::new(m_type.short_code()),
                    TextFont {
                        font_size: 9.0.into(),
                        ..default()
                    },
                    TextColor(color),
                    Transform::from_xyz(0.0, custom_size.y / 1.4, LABEL_Z - MACHINE_Z),
                ));
            });

            match m_type {
                MachineType::Reactor => {
                    machine_cmd.insert((
                        ThermalThreshold {
                            critical: REACTOR_CRITICAL_TEMP,
                            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
                        },
                        Vulnerabilities(vec![VulnerabilityType::HullBreach]),
                    ));
                }
                MachineType::LifeSupport => {
                    machine_cmd.insert((
                        LifeSupport {
                            output_rate: OXYGEN_REPLENISH_RATE,
                        },
                        ThermalThreshold {
                            critical: DEFAULT_CRITICAL_TEMP,
                            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                        },
                        Vulnerabilities(vec![VulnerabilityType::PowerShortage]),
                    ));
                }
                _ => {
                    machine_cmd.insert((
                        ThermalThreshold {
                            critical: DEFAULT_CRITICAL_TEMP,
                            timer: Timer::from_seconds(4.0, TimerMode::Repeating),
                        },
                        Vulnerabilities(vec![
                            VulnerabilityType::PowerShortage,
                            VulnerabilityType::DoorMalfunction,
                        ]),
                    ));
                }
            }
        }
    }
}

fn split_rect(rect: Rect, depth: u32, rng: &mut ThreadRng, rects: &mut Vec<Rect>) {
    let w = rect.width();
    let h = rect.height();

    if depth == 0 || (w < MIN_ROOM_SIZE * 2.0 && h < MIN_ROOM_SIZE * 2.0) {
        rects.push(rect);
        return;
    }

    let split_horiz = if w < MIN_ROOM_SIZE * 2.0 {
        true
    } else if h < MIN_ROOM_SIZE * 2.0 {
        false
    } else {
        rng.random_bool(0.5)
    };

    if split_horiz {
        let split_y = rng.random_range((rect.min.y + MIN_ROOM_SIZE)..(rect.max.y - MIN_ROOM_SIZE));
        split_rect(
            Rect::new(rect.min.x, rect.min.y, rect.max.x, split_y),
            depth - 1,
            rng,
            rects,
        );
        split_rect(
            Rect::new(rect.min.x, split_y, rect.max.x, rect.max.y),
            depth - 1,
            rng,
            rects,
        );
    } else {
        let split_x = rng.random_range((rect.min.x + MIN_ROOM_SIZE)..(rect.max.x - MIN_ROOM_SIZE));
        split_rect(
            Rect::new(rect.min.x, rect.min.y, split_x, rect.max.y),
            depth - 1,
            rng,
            rects,
        );
        split_rect(
            Rect::new(split_x, rect.min.y, rect.max.x, rect.max.y),
            depth - 1,
            rng,
            rects,
        );
    }
}

fn spawn_doors_on_edges(commands: &mut Commands, rooms: &[(Entity, Rect)]) {
    let mut door_count = 1;

    for i in 0..rooms.len() {
        for j in (i + 1)..rooms.len() {
            let (ent_a, rect_a) = &rooms[i];
            let (ent_b, rect_b) = &rooms[j];

            let is_touching_x = (rect_a.max.x - rect_b.min.x).abs() < 1.0
                || (rect_a.min.x - rect_b.max.x).abs() < 1.0;
            let overlap_y = rect_a.min.y.max(rect_b.min.y) < rect_a.max.y.min(rect_b.max.y);

            let is_touching_y = (rect_a.max.y - rect_b.min.y).abs() < 1.0
                || (rect_a.min.y - rect_b.max.y).abs() < 1.0;
            let overlap_x = rect_a.min.x.max(rect_b.min.x) < rect_a.max.x.min(rect_b.max.x);

            let mut door_pos = None;

            if is_touching_x && overlap_y {
                let x = if (rect_a.max.x - rect_b.min.x).abs() < 1.0 {
                    rect_a.max.x
                } else {
                    rect_a.min.x
                };
                let y = (rect_a.min.y.max(rect_b.min.y) + rect_a.max.y.min(rect_b.max.y)) / 2.0;
                door_pos = Some(Vec2::new(x, y));
            } else if is_touching_y && overlap_x {
                let y = if (rect_a.max.y - rect_b.min.y).abs() < 1.0 {
                    rect_a.max.y
                } else {
                    rect_a.min.y
                };
                let x = (rect_a.min.x.max(rect_b.min.x) + rect_a.max.x.min(rect_b.max.x)) / 2.0;
                door_pos = Some(Vec2::new(x, y));
            }

            if let Some(pos) = door_pos {
                let door_name = format!("D{:02}", door_count);
                let color = Color::srgb(0.2, 0.9, 0.2);

                commands
                    .spawn((
                        Door {
                            id_name: door_name.clone(),
                            is_open: true,
                            room_a: *ent_a,
                            room_b: *ent_b,
                        },
                        Transform::from_xyz(pos.x, pos.y, DOOR_Z),
                        DoorVisual,
                        Sprite {
                            color,
                            custom_size: Some(Vec2::new(12.0, 12.0)),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            DoorLabel,
                            Text2d::new(door_name),
                            TextFont {
                                font_size: 10.0.into(),
                                ..default()
                            },
                            TextColor(color),
                            Transform::from_xyz(0.0, 11., LABEL_Z - DOOR_Z),
                        ));
                    });
                door_count += 1;
            }
        }
    }
}
