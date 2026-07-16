use super::components::*;
use crate::simulation::components::{Door, Neighbors};
use bevy::{math::Isometry2d, prelude::*};

pub fn render_map(
    query: Query<(Entity, &Transform, &RoomLayout, &Neighbors)>,
    door_query: Query<(&Transform, &Door)>,
    config: Res<MapGenConfig>,
    mut gizmos: Gizmos,
) {
    gizmos.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(40, 40),
        Vec2::splat(config.grid_size),
        config.background_color,
    );

    for (entity, transform, layout, neighbors) in &query {
        let center = transform.translation.truncate();
        let half_size = Vec2::new(layout.width / 2.0, layout.height / 2.0);

        gizmos.rect_2d(
            Isometry2d::from_translation(center),
            half_size,
            config.room_color,
        );

        for &neighbor_ent in &neighbors.0 {
            if let Ok((_, n_transform, _, _)) = query.get(neighbor_ent)
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
