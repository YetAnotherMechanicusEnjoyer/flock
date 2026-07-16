use crate::utils::consts::{MAX_TEMP, ZERO_CELSIUS};

use super::components::*;
use bevy::prelude::*;

pub fn spawn_initial_ship(mut commands: Commands) {
    let reactor_id = commands.spawn_empty().id();
    let hallway_id = commands.spawn_empty().id();
    let bridge_id = commands.spawn_empty().id();
    let target = ZERO_CELSIUS + 20.0;

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
