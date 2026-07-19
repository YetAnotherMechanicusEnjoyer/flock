use crate::{
    terminal::components::PrintToTerminal,
    utils::consts::{
        HEAT_DELTA_ACTIVE, HEAT_DELTA_OFFLINE, HEAT_DELTA_OVERCHARGE, MAX_TEMP, OXYGEN_MAX,
        OXYGEN_VACUUM_LEAK,
    },
};

use super::components::*;
use bevy::prelude::*;

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
