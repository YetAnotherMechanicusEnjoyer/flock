pub mod components;
pub mod generator;
pub mod render;

use crate::core::state::AppState;
use bevy::prelude::*;
use components::MapGenConfig;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapGenConfig>()
            .add_systems(
                OnEnter(AppState::ActiveSimulation),
                generator::spawn_procedural_ship,
            )
            .add_systems(
                Update,
                render::render_map.run_if(in_state(AppState::ActiveSimulation)),
            );
    }
}
