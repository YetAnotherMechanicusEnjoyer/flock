pub mod components;
pub mod parser;
pub mod systems;

use crate::{
    core::state::AppState,
    simulation::components::{Door, HullIntegrity, PowerState, RepairTask, Room, Temperature},
    terminal::components::PrintToTerminal,
    utils::convert::kelvin_to_celsius,
};
use bevy::prelude::*;

use components::CommandQueue;
use parser::parse_command;

pub type RoomQuery<'a> = (
    Entity,
    &'a Room,
    &'a Temperature,
    &'a mut PowerState,
    &'a mut HullIntegrity,
    Option<&'a RepairTask>,
);

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandQueue>()
            .add_message::<PrintToTerminal>()
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
                    systems::terminal_scroll,
                )
                    .run_if(in_state(AppState::ActiveSimulation)),
            );
    }
}

fn process_commands(
    mut commands: Commands,
    mut command_queue: ResMut<CommandQueue>,
    mut printer: MessageWriter<PrintToTerminal>,
    mut room_query: Query<RoomQuery>,
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
            parser::ParsedCommand::Repair(target) => {
                handle_repair(&mut commands, &mut printer, &mut room_query, &target);
            }
            parser::ParsedCommand::Unknown(cmd) => {
                printer.write(PrintToTerminal(format!("COMMAND NOT FOUND: {}", cmd)));
            }
            parser::ParsedCommand::Empty => {}
        }
    }
}

fn handle_status(printer: &mut MessageWriter<PrintToTerminal>, room_query: &Query<RoomQuery>) {
    printer.write(PrintToTerminal("--- SHIP STATUS ---".to_string()));
    for (entity, room, temp, power, hull, task) in room_query {
        let temp_c = kelvin_to_celsius(temp.current);
        let repair_status = if task.is_some() { "[REPAIRING]" } else { "" };
        let status_line = format!(
            "{entity:?} | Room: {} | Temp: {:.1}°C | Power: {:?} | Hull: {}% {}",
            room.name, temp_c, power, hull.0, repair_status
        );
        printer.write(PrintToTerminal(status_line));
    }
}

fn handle_help(printer: &mut MessageWriter<PrintToTerminal>) {
    printer.write(PrintToTerminal(
        "AVAILABLE COMMANDS:\n* status\n* help\n* door <id>\n* power <room> <off|on|over>\n* repair <room>".to_string(),
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
    room_query: &mut Query<RoomQuery>,
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

    for (_, room, _, mut power, _, _) in room_query.iter_mut() {
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

fn handle_repair(
    commands: &mut Commands,
    printer: &mut MessageWriter<PrintToTerminal>,
    room_query: &mut Query<RoomQuery>,
    target_room: &str,
) {
    for (entity, room, _, _, hull, repair_task) in room_query.iter_mut() {
        if room
            .name
            .to_lowercase()
            .contains(&target_room.to_lowercase())
        {
            if repair_task.is_some() {
                printer.write(PrintToTerminal(format!(
                    "ERROR: Repair drones are already deployed in {}.",
                    room.name
                )));
                return;
            }
            if hull.0 >= 100.0 {
                printer.write(PrintToTerminal(format!(
                    "INFO: Hull is already at optimal integrity in {}.",
                    room.name
                )));
                return;
            }

            commands
                .entity(entity)
                .insert(RepairTask(Timer::from_seconds(10.0, TimerMode::Once)));
            printer.write(PrintToTerminal(format!(
                "DEPLOYING DRONES: Commencing hull repairs in {}. ETA: 10s.",
                room.name
            )));
            return;
        }
    }
    printer.write(PrintToTerminal(format!(
        "ERROR: ROOM MATCHING '{target_room}' NOT FOUND."
    )));
}
