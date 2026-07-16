use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ParsedCommand {
    Status,
    Help,
    Unknown(String),
    Empty,
}

pub fn parse_command(input: &str) -> ParsedCommand {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return ParsedCommand::Empty;
    }

    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next().unwrap_or_default();

    match cmd {
        "status" => ParsedCommand::Status,
        "help" => ParsedCommand::Help,
        _ => ParsedCommand::Unknown(cmd.to_string()),
    }
}
