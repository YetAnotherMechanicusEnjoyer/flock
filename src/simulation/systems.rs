use crate::{
    map::components::RoomLayout,
    utils::consts::{MAX_TEMP, ZERO_CELSIUS},
};

use super::components::*;
use bevy::prelude::*;

pub fn spawn_initial_ship(mut commands: Commands) {
    let reactor_id = commands.spawn_empty().id();
    let hallway_id = commands.spawn_empty().id();
    let bridge_id = commands.spawn_empty().id();
    let target = ZERO_CELSIUS;

    commands.entity(reactor_id).insert((
        Room {
            name: "Reactor Core",
            volume: 100.0,
        },
        Temperature {
            current: ZERO_CELSIUS + 800.0,
            target,
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
            current: ZERO_CELSIUS + 20.0,
            target,
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
            current: ZERO_CELSIUS + 20.0,
            target,
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

    commands.spawn((
        Door {
            id_name: "D1".to_string(),
            is_open: false,
            room_a: reactor_id,
            room_b: hallway_id,
        },
        Transform::from_xyz(-150.0, 50.0, 0.0),
    ));

    commands.spawn((
        Door {
            id_name: "D2".to_string(),
            is_open: true,
            room_a: hallway_id,
            room_b: bridge_id,
        },
        Transform::from_xyz(150.0, 50.0, 0.0),
    ));

    info!("Ship topology generated: Reactor <-> Hallway <-> Bridge");
}

pub fn process_thermodynamics(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Temperature, &PowerState)>,
) {
    let delta = time.delta_secs();

    for (mut temp, power) in query.iter_mut() {
        let heat_delta = match power {
            PowerState::Offline => -1.5,
            PowerState::Active => 0.5,
            PowerState::Overcharged => 15.0,
        };

        if heat_delta != 0.0 {
            temp.current = (temp.current + (heat_delta * delta)).clamp(0.0, MAX_TEMP);
        }
    }
}
