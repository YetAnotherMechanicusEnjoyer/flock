mod core;
mod map;
//mod render;
mod simulation;
mod terminal;
#[allow(unused)]
mod utils;

use bevy::{prelude::*, window::WindowResolution};
use core::state::AppState;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flock - Terminal".into(),
                resolution: WindowResolution::new(1280, 720),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            map::MapPlugin,
            simulation::SimulationPlugin,
            terminal::TerminalPlugin,
        ))
        .add_systems(Startup, |mut next_state: ResMut<NextState<AppState>>| {
            next_state.set(AppState::ActiveSimulation);
        })
        .run();
}
