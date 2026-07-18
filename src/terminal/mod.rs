pub mod components;
pub mod parser;
pub mod systems;

use crate::{
    core::state::AppState,
    simulation::components::{Door, PowerState, Room, Temperature},
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
                    systems::terminal_scroll,
                )
                    .run_if(in_state(AppState::ActiveSimulation)),
            );
    }
}

fn update_measures(
    mut printer: MessageWriter<PrintToMeasures>,
    room_query: Query<(&Room, &Temperature, &PowerState)>,
) {
    let mut room_cache: Vec<&str> = Vec::new();
    for (room, temp, power) in &room_query {
        if room_cache.contains(&room.name) {
            continue;
        }
        let temp_c = kelvin_to_celsius(temp.current);

        let status_line = format!("[{}] Temp: {:.1}°C | Power: {:?}", room.name, temp_c, power);
        printer.write(PrintToMeasures(status_line));
        room_cache.push(room.name);
    }
}

fn process_commands(
    mut command_queue: ResMut<CommandQueue>,
    mut printer: MessageWriter<PrintToTerminal>,
    mut room_query: Query<(Entity, &Room, &Temperature, &mut PowerState)>,
    mut door_query: Query<&mut Door>,
) {
    if command_queue.pending.is_empty() {
        return;
    }

    for cmd_str in command_queue.pending.drain(..) {
        let parsed = parse_command(&cmd_str);

        printer.write(PrintToTerminal(format!("> {}", cmd_str)));

        match parsed {
            parser::ParsedCommand::Status => handle_status(&mut printer, &room_query),
            parser::ParsedCommand::Help => handle_help(&mut printer),
            parser::ParsedCommand::ToggleDoor(door_id) => {
                handle_door(&mut printer, &mut door_query, &door_id)
            }
            parser::ParsedCommand::SetPower(target, state) => {
                handle_power(&mut printer, &mut room_query, &target, &state);
            }
            parser::ParsedCommand::Unknown(cmd) => {
                printer.write(PrintToTerminal(format!("COMMAND NOT FOUND: {}", cmd)));
            }
            parser::ParsedCommand::Empty => {}
        }
    }
}

fn handle_status(
    printer: &mut MessageWriter<PrintToTerminal>,
    room_query: &Query<(Entity, &Room, &Temperature, &mut PowerState)>,
) {
    printer.write(PrintToTerminal("--- SHIP STATUS ---".to_string()));
    for (entity, room, temp, power) in room_query {
        let temp_c = kelvin_to_celsius(temp.current);
        let status_line = format!(
            "{entity:?} | Room: {} | Temp: {:.1}°C | Power: {:?}",
            room.name, temp_c, power
        );
        printer.write(PrintToTerminal(status_line));
    }
}

fn handle_help(printer: &mut MessageWriter<PrintToTerminal>) {
    printer.write(PrintToTerminal(
        "AVAILABLE COMMANDS: status, help, door <id>, power <room> <on|off|over>".to_string(),
    ));
}

fn handle_door(
    printer: &mut MessageWriter<PrintToTerminal>,
    door_query: &mut Query<&mut Door>,
    door_id: &str,
) {
    for mut door in door_query.iter_mut() {
        if door.id_name.eq_ignore_ascii_case(door_id) {
            door.is_open = !door.is_open;
            let state = if door.is_open { "OPENED" } else { "CLOSED" };
            printer.write(PrintToTerminal(format!(
                "DOOR {} IS NOW {state}.",
                door.id_name
            )));
            return;
        }
    }
    printer.write(PrintToTerminal(format!(
        "ERROR: DOOR '{door_id}' NOT FOUND."
    )));
}

fn handle_power(
    printer: &mut MessageWriter<PrintToTerminal>,
    room_query: &mut Query<(Entity, &Room, &Temperature, &mut PowerState)>,
    target_room: &str,
    state_str: &str,
) {
    let new_state = match state_str.to_lowercase().as_str() {
        "off" | "offline" => PowerState::Offline,
        "on" | "active" => PowerState::Active,
        "over" | "overcharge" => PowerState::Overcharged,
        _ => {
            printer.write(PrintToTerminal(format!(
                "ERROR: INVALID POWER STATE '{state_str}'. USE: on, off, over"
            )));
            return;
        }
    };

    for (_, room, _, mut power) in room_query.iter_mut() {
        if room
            .name
            .to_lowercase()
            .contains(&target_room.to_lowercase())
        {
            *power = new_state;
            printer.write(PrintToTerminal(format!(
                "POWER ROUTING: {} is now {:?}",
                room.name, new_state
            )));
            return;
        }
    }
    printer.write(PrintToTerminal(format!(
        "ERROR: ROOM MATCHING '{target_room}' NOT FOUND."
    )));
}
