use super::components::*;
use crate::{
    simulation::components::{Door, Neighbors, Room, Temperature},
    utils::convert::kelvin_to_celsius,
};
use bevy::{math::Isometry2d, prelude::*};

pub fn render_map(
    query: Query<(
        Entity,
        &Transform,
        &Room,
        &RoomLayout,
        &Temperature,
        &Neighbors,
    )>,
    door_query: Query<(&Transform, &Door)>,
    config: Res<MapGenConfig>,
    mut gizmos: Gizmos,
) {
    for (entity, transform, room, layout, temperature, neighbors) in &query {
        let center = transform.translation.truncate();
        let half_size = Vec2::new(layout.width / 2.0, layout.height / 2.0);

        gizmos.rect_2d(
            Isometry2d::from_translation(center),
            half_size,
            config.room_color,
        );

        gizmos.text_2d(
            Isometry2d::from_translation(Vec2::new(
                center.x,
                (center.y + layout.height / 4.0) + 15.,
            )),
            room.name,
            10.0,
            Vec2::new(0., 0.),
            config.room_color,
        );

        gizmos.text_2d(
            Isometry2d::from_translation(Vec2::new(
                center.x,
                (center.y - layout.height / 4.0) - 15.,
            )),
            format!("{:.1}°C", kelvin_to_celsius(temperature.current)).as_str(),
            10.0,
            Vec2::new(0., 0.),
            config.room_color,
        );

        for &neighbor_ent in &neighbors.0 {
            if let Ok((_, n_transform, _, _, _, _)) = query.get(neighbor_ent)
                && entity.index() < neighbor_ent.index()
            {
                let n_center = n_transform.translation.truncate();

                gizmos.line_2d(center, n_center, config.corridor_color);
            }
        }
    }

    for (transform, door) in door_query.iter() {
        let center = transform.translation.truncate();
        let color = if door.is_open {
            Color::srgb(0.2, 0.9, 0.2)
        } else {
            Color::srgb(0.9, 0.2, 0.2)
        };

        let size = Vec2::new(16., 16.);

        gizmos.rect_2d(Isometry2d::from_translation(center), size, color);

        gizmos.text_2d(
            Isometry2d::from_translation(center),
            &door.id_name,
            10.0,
            Vec2::new(0., -1.5),
            color,
        );
    }
}
