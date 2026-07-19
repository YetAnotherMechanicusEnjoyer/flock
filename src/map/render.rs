use super::components::*;
use crate::simulation::components::*;
use bevy::prelude::*;

type MachineQuery<'a> = (&'a PowerState, &'a mut Sprite, &'a Children);
type MachineFilter = (
    With<MachineVisual>,
    Without<DoorVisual>,
    Changed<PowerState>,
);
type DoorQuery<'a> = (&'a Door, &'a mut Sprite, &'a Children);
type DoorFilter = (With<DoorVisual>, Without<MachineVisual>, Changed<Door>);

pub fn render_map(
    mut machine_query: Query<MachineQuery, MachineFilter>,
    mut door_query: Query<DoorQuery, DoorFilter>,
    mut text_color_query: Query<&mut TextColor>,
) {
    for (power, mut sprite, children) in &mut machine_query {
        let color = match power {
            PowerState::Offline => Color::srgb(0.3, 0.3, 0.3),
            PowerState::Active => Color::srgb(0.2, 0.8, 0.2),
            PowerState::Overcharged => Color::srgb(1.0, 0.2, 0.2),
        };
        sprite.color = color;

        for &child in children {
            if let Ok(mut text_color) = text_color_query.get_mut(child) {
                text_color.0 = color;
            }
        }
    }

    for (door, mut sprite, children) in &mut door_query {
        let color = if door.is_open {
            Color::srgb(0.2, 0.6, 0.2)
        } else {
            Color::srgb(0.6, 0.2, 0.2)
        };
        sprite.color = color;

        for &child in children {
            if let Ok(mut text_color) = text_color_query.get_mut(child) {
                text_color.0 = color;
            }
        }
    }
}
