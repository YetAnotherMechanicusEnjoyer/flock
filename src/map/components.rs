use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct MapGenConfig {
    pub grid_size: f32,
    pub room_color: Color,
    pub corridor_color: Color,
    pub background_color: Color,
}

impl Default for MapGenConfig {
    fn default() -> Self {
        Self {
            grid_size: 64.0,
            room_color: Color::srgb(0.9, 0.5, 0.1),
            corridor_color: Color::srgb(0.4, 0.2, 0.05),
            background_color: Color::srgb(0.02, 0.02, 0.02),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct RoomLayout {
    pub width: f32,
    pub height: f32,
}
