use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    BootSequence,
    ActiveSimulation,
    SystemFailure,
}
