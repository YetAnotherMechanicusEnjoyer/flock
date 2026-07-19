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

#[derive(Component)]
pub struct RoomVisual;

#[derive(Component)]
pub struct MachineVisual;

#[derive(Component)]
pub struct DoorVisual;

#[derive(Component)]
pub struct RoomLabel;

#[derive(Component)]
pub struct MachineLabel;

#[derive(Component)]
pub struct DoorLabel;

pub const GRID_Z: f32 = -20.0;
pub const ROOM_Z: f32 = 0.0;
pub const CORRIDOR_Z: f32 = 2.0;
pub const MACHINE_Z: f32 = 5.0;
pub const DOOR_Z: f32 = 10.0;
pub const LABEL_Z: f32 = 20.0;
