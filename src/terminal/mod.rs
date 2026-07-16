pub mod components;
pub mod parser;
pub mod systems;

use crate::{
    core::state::AppState,
    simulation::components::{PowerState, Room, Temperature},
    terminal::components::{PrintToMeasures, PrintToTerminal},
    utils::convert::kelvin_to_celsius,
};
use bevy::prelude::*;

use components::CommandQueue;
use parser::parse_command;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandQueue>()
            .add_message::<PrintToTerminal>()
            .add_message::<PrintToMeasures>()
            .add_systems(
                OnEnter(AppState::ActiveSimulation),
                systems::setup_terminal_ui,
            )
            .add_systems(
                Update,
                (
                    process_commands,
                    systems::handle_typing,
                    systems::update_terminal_history,
                    update_measures,
                    systems::update_measures,
                )
                    .run_if(in_state(AppState::ActiveSimulation)),
            );
    }
}

fn update_measures(
    mut printer: MessageWriter<PrintToMeasures>,
    room_query: Query<(&Room, &Temperature, &PowerState)>,
) {
    for (room, temp, power) in &room_query {
        let temp_c = kelvin_to_celsius(temp.current);

        let status_line = format!("[{}] Temp: {:.1}°C | Power: {:?}", room.name, temp_c, power);
        printer.write(PrintToMeasures(status_line));
    }
}

fn process_commands(
    mut command_queue: ResMut<CommandQueue>,
    mut printer: MessageWriter<PrintToTerminal>,
    room_query: Query<(&Room, &Temperature, &PowerState)>,
) {
    if command_queue.pending.is_empty() {
        return;
    }

    for cmd_str in command_queue.pending.drain(..) {
        let parsed = parse_command(&cmd_str);

        printer.write(PrintToTerminal(format!("> {}", cmd_str)));

        match parsed {
            parser::ParsedCommand::Status => {
                printer.write(PrintToTerminal("--- SHIP STATUS ---".to_string()));

                for (room, temp, power) in &room_query {
                    let temp_c = kelvin_to_celsius(temp.current);

                    let status_line =
                        format!("[{}] Temp: {:.1}°C | Power: {:?}", room.name, temp_c, power);
                    printer.write(PrintToTerminal(status_line));
                }
            }
            parser::ParsedCommand::Help => {
                printer.write(PrintToTerminal(
                    "AVAILABLE COMMANDS: status, help".to_string(),
                ));
            }
            parser::ParsedCommand::Unknown(cmd) => {
                printer.write(PrintToTerminal(format!("COMMAND NOT FOUND: {}", cmd)));
            }
            parser::ParsedCommand::Empty => {}
        }
    }
}
