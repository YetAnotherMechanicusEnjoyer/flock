use bevy::prelude::*;

use crate::{
    simulation::components::{Neighbors, Temperature, ThermalDelta},
    utils::consts::MAX_TEMP,
};

pub fn calculate_heat_transfer(
    query: Query<(Entity, &Temperature, &Neighbors)>,
    temp_query: Query<&Temperature>,
    mut delta_query: Query<&mut ThermalDelta>,
) {
    let transfer_rate = 0.1;

    for (entity, temp, neighbors) in query.iter() {
        let mut total_heat_exchange = 0.0;

        for &neighbor_entity in neighbors.0.iter() {
            if let Ok(neighbor_temp) = temp_query.get(neighbor_entity) {
                let difference = neighbor_temp.current - temp.current;
                total_heat_exchange += difference * transfer_rate;
            }
        }

        if let Ok(mut delta) = delta_query.get_mut(entity) {
            delta.0 += total_heat_exchange;
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
