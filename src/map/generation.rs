use super::components::*;
use crate::simulation::components::*;
use bevy::prelude::*;

pub fn generate_ship_layout(mut commands: Commands) {
    let reactor_id = commands.spawn_empty().id();
    let hallway_id = commands.spawn_empty().id();
    let bridge_id = commands.spawn_empty().id();

    commands.entity(reactor_id).insert((
        Room {
            name: "Reactor Core",
            volume: 100.0,
        },
        Temperature {
            current: 1000.0,
            target: 293.15,
        },
        ThermalDelta(0.0),
        PowerState::Active,
        Neighbors(vec![hallway_id]),
        Transform::from_xyz(-300.0, 50.0, 0.0),
        RoomLayout {
            width: 120.0,
            height: 120.0,
        },
    ));

    commands.entity(hallway_id).insert((
        Room {
            name: "Main Hallway",
            volume: 50.0,
        },
        Temperature {
            current: 293.15,
            target: 293.15,
        },
        ThermalDelta(0.0),
        PowerState::Active,
        Neighbors(vec![reactor_id, bridge_id]),
        Transform::from_xyz(0.0, 50.0, 0.0),
        RoomLayout {
            width: 300.0,
            height: 60.0,
        },
    ));

    commands.entity(bridge_id).insert((
        Room {
            name: "Bridge",
            volume: 200.0,
        },
        Temperature {
            current: 293.15,
            target: 293.15,
        },
        ThermalDelta(0.0),
        PowerState::Active,
        Neighbors(vec![hallway_id]),
        Transform::from_xyz(300.0, 50.0, 0.0),
        RoomLayout {
            width: 150.0,
            height: 180.0,
        },
    ));
}
