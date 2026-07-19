use super::components::*;
use crate::simulation::components::*;
use bevy::{math::Isometry2d, prelude::*};

pub fn render_map(
    room_query: Query<(&Transform, &Room, &RoomLayout)>,
    machine_query: Query<(&Transform, &Machine, &PowerState)>,
    door_query: Query<(&Transform, &Door)>,
    config: Res<MapGenConfig>,
    mut gizmos: Gizmos,
) {
    for (transform, room, layout) in &room_query {
        let center = transform.translation.truncate();
        let half_size = Vec2::new(layout.width, layout.height);

        gizmos.rect_2d(
            Isometry2d::from_translation(center),
            half_size,
            config.room_color,
        );

        gizmos.text_2d(
            Isometry2d::from_translation(Vec2::new(
                center.x,
                center.y + layout.height / 2.0 - 10.0,
            )),
            &room.name,
            12.0,
            Vec2::new(0., 0.),
            config.room_color,
        );
    }

    for (transform, machine, power) in &machine_query {
        let pos = transform.translation.truncate();
        let size = Vec2::splat(crate::utils::consts::MACHINE_RENDER_SIZE);

        let color = match power {
            PowerState::Offline => Color::srgb(0.3, 0.3, 0.3),
            PowerState::Active => Color::srgb(0.2, 0.8, 0.2),
            PowerState::Overcharged => Color::srgb(1.0, 0.2, 0.2),
        };

        match machine.machine_type {
            MachineType::Reactor => {
                gizmos.circle_2d(pos, size.x / 2.0, color);
            }
            MachineType::LifeSupport => {
                gizmos.rect_2d(Isometry2d::from_translation(pos), size, color);
            }
            MachineType::Server => {
                gizmos.ellipse_2d(
                    Isometry2d::from_translation(pos),
                    Vec2::new(size.x / 2.0, size.y),
                    color,
                );
            }
            MachineType::Cooler => {
                gizmos.circle_2d(Isometry2d::from_translation(pos), size.x / 2.0, color);
                gizmos.cross_2d(Isometry2d::from_translation(pos), size.x / 2.0, color);
            }
        }

        gizmos.text_2d(
            Isometry2d::from_translation(pos),
            machine.machine_type.short_code(),
            9.0,
            Vec2::new(0., -1.5),
            color,
        );
    }

    for (transform, door) in &door_query {
        let center = transform.translation.truncate();
        let color = if door.is_open {
            Color::srgb(0.2, 0.9, 0.2)
        } else {
            Color::srgb(0.9, 0.2, 0.2)
        };

        gizmos.rect_2d(
            Isometry2d::from_translation(center),
            Vec2::new(12., 12.),
            color,
        );

        gizmos.text_2d(
            Isometry2d::from_translation(center),
            &door.id_name,
            10.0,
            Vec2::new(0., -1.5),
            color,
        );
    }
}
