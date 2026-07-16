use crate::core::state::AppState;
use bevy::prelude::*;

pub mod components;
pub mod propagation;
pub mod systems;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0));

        app.add_systems(
            OnEnter(AppState::ActiveSimulation),
            systems::spawn_initial_ship,
        )
        .add_systems(
            FixedUpdate,
            (
                propagation::calculate_heat_transfer,
                propagation::apply_thermal_deltas,
            )
                .chain()
                .run_if(in_state(AppState::ActiveSimulation)),
        );
    }
}
