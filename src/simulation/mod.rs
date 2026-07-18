use crate::{core::state::AppState, simulation::events::CriticalTemperatureEvent};
use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod propagation;
pub mod systems;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
            .add_message::<CriticalTemperatureEvent>();

        app.add_systems(
            OnEnter(AppState::ActiveSimulation),
            systems::spawn_initial_ship,
        )
        .add_systems(
            FixedUpdate,
            (
                systems::process_thermodynamics,
                systems::process_life_support,
                propagation::calculate_heat_transfer,
                propagation::apply_thermal_deltas,
                propagation::calculate_oxygen_transfer,
                propagation::apply_oxygen_deltas,
                events::detect_critical_temperatures,
                events::resolve_critical_events,
                systems::process_repairs,
            )
                .chain()
                .run_if(in_state(AppState::ActiveSimulation)),
        );
    }
}
