use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct MapGenConfig {
    pub room_color: Color,
    pub corridor_color: Color,
}

impl Default for MapGenConfig {
    fn default() -> Self {
        Self {
            room_color: Color::srgb(0.9, 0.5, 0.1),
            corridor_color: Color::srgb(0.4, 0.2, 0.05),
        }
    }
}
