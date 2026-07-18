use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ParsedCommand {
    Status,
    Help,
    ToggleDoor(String),
    SetPower(String, String),
    Unknown(String),
    Empty,
}

pub fn parse_command(input: &str) -> ParsedCommand {
    let trimmed = input.trim().to_lowercase();

    if trimmed.is_empty() {
        return ParsedCommand::Empty;
    }

    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next().unwrap_or_default();

    match cmd {
        "status" => ParsedCommand::Status,
        "help" => ParsedCommand::Help,
        "door" => ParsedCommand::ToggleDoor(parts.next().unwrap_or_default().to_string()),
        "power" => ParsedCommand::SetPower(
            parts.next().unwrap_or_default().to_string(),
            parts.next().unwrap_or_default().to_string(),
        ),
        _ => ParsedCommand::Unknown(cmd.to_string()),
    }
}
