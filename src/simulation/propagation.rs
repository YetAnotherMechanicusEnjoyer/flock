use bevy::prelude::*;

use crate::{
    simulation::components::{Door, Temperature, ThermalDelta},
    utils::consts::MAX_TEMP,
};

pub fn calculate_heat_transfer(
    door_query: Query<&Door>,
    temp_query: Query<&Temperature>,
    mut delta_query: Query<&mut ThermalDelta>,
) {
    let transfer_rate = 0.5;

    for door in door_query.iter() {
        if !door.is_open {
            continue;
        }

        if let (Ok(temp_a), Ok(temp_b)) = (temp_query.get(door.room_a), temp_query.get(door.room_b))
        {
            let difference = temp_b.current - temp_a.current;
            let exchange = difference * transfer_rate;

            if let Ok(mut delta_a) = delta_query.get_mut(door.room_a) {
                delta_a.0 += exchange;
            }
            if let Ok(mut delta_b) = delta_query.get_mut(door.room_b) {
                delta_b.0 -= exchange;
            }
        }
    }
}

pub fn apply_thermal_deltas(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Temperature, &mut ThermalDelta)>,
) {
    let dt = time.delta_secs();

    for (mut temp, mut delta) in query.iter_mut() {
        if delta.0 != 0.0 {
            temp.current = (temp.current + (delta.0 * dt)).clamp(0.0, MAX_TEMP);
            delta.0 = 0.0;
        }
    }
}
