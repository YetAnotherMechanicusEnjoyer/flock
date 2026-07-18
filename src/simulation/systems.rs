use crate::{
    map::components::RoomLayout,
    terminal::components::PrintToTerminal,
    utils::consts::{
        DEFAULT_CRITICAL_TEMP, HEAT_DELTA_ACTIVE, HEAT_DELTA_OFFLINE, HEAT_DELTA_OVERCHARGE,
        MAX_TEMP, OXYGEN_MAX, OXYGEN_REPLENISH_RATE, OXYGEN_VACUUM_LEAK, REACTOR_CRITICAL_TEMP,
        ZERO_CELSIUS,
    },
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
        HullIntegrity(100.0),
        ThermalThreshold {
            critical: REACTOR_CRITICAL_TEMP,
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        },
        Vulnerabilities(vec![
            VulnerabilityType::PowerShortage,
            VulnerabilityType::HullBreach,
        ]),
        Oxygen(OXYGEN_MAX),
        OxygenDelta(0.0),
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
        HullIntegrity(100.0),
        ThermalThreshold {
            critical: DEFAULT_CRITICAL_TEMP,
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        },
        Vulnerabilities(vec![VulnerabilityType::DoorMalfunction]),
        Oxygen(OXYGEN_MAX),
        OxygenDelta(0.0),
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
        HullIntegrity(100.0),
        ThermalThreshold {
            critical: DEFAULT_CRITICAL_TEMP,
            timer: Timer::from_seconds(4.0, TimerMode::Repeating),
        },
        Vulnerabilities(vec![
            VulnerabilityType::PowerShortage,
            VulnerabilityType::DoorMalfunction,
            VulnerabilityType::HullBreach,
        ]),
        Oxygen(OXYGEN_MAX),
        OxygenDelta(0.0),
        LifeSupport {
            output_rate: OXYGEN_REPLENISH_RATE,
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
    mut query: Query<(&mut Temperature, &PowerState, &HullIntegrity)>,
) {
    let delta = time.delta_secs();

    for (mut temp, power, hull) in query.iter_mut() {
        let mut heat_delta = match power {
            PowerState::Offline => HEAT_DELTA_OFFLINE,
            PowerState::Active => HEAT_DELTA_ACTIVE,
            PowerState::Overcharged => HEAT_DELTA_OVERCHARGE,
        };

        if hull.0 < 10.0 {
            let leak_factor = (100.0 - hull.0) / 100.0;
            heat_delta -= leak_factor * 50.0;
        }

        if heat_delta != 0.0 {
            temp.current = (temp.current + (heat_delta * delta)).clamp(0.0, MAX_TEMP);
        }
    }
}

pub fn process_repairs(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    mut query: Query<(Entity, &Room, &mut HullIntegrity, &mut RepairTask)>,
    mut printer: MessageWriter<PrintToTerminal>,
) {
    for (entity, room, mut hull, mut task) in query.iter_mut() {
        task.0.tick(time.delta());

        if task.0.just_finished() {
            hull.0 = 100.0;
            commands.entity(entity).remove::<RepairTask>();
            printer.write(PrintToTerminal(format!(
                "DRONES: Hull integrity fully restored in {}.",
                room.name
            )));
        }
    }
}

pub fn process_life_support(
    time: Res<Time<Fixed>>,
    ls_query: Query<(&LifeSupport, &PowerState)>,
    mut room_query: Query<(&mut Oxygen, &HullIntegrity)>,
) {
    let dt = time.delta_secs();

    let mut global_replenishment = 0.0;
    for (ls, power) in ls_query.iter() {
        match power {
            PowerState::Active => global_replenishment += ls.output_rate,
            PowerState::Overcharged => global_replenishment += ls.output_rate * 2.5,
            PowerState::Offline => {}
        }
    }

    for (mut o2, hull) in room_query.iter_mut() {
        let mut delta = 0.0;

        if o2.0 < OXYGEN_MAX {
            delta += global_replenishment;
        }

        if hull.0 < 100.0 {
            let leak_factor = (100.0 - hull.0) / 100.0;
            delta -= leak_factor * OXYGEN_VACUUM_LEAK;
        }

        if delta != 0.0 {
            o2.0 = (o2.0 + delta * dt).clamp(0.0, OXYGEN_MAX);
        }
    }
}
