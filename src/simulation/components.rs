use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Room {
    pub name: String,
}

#[derive(Component, Debug, Clone)]
pub struct RoomLayout {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Debug, Clone)]
pub struct Neighbors(pub Vec<Entity>);

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ThermalDelta(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

#[derive(Component, Debug, Clone, Copy)]
pub struct Temperature {
    pub current: f32,
    pub target: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Offline,
    Active,
    Overcharged,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct HullIntegrity(pub f32);

#[derive(Component, Debug, Clone)]
pub struct Door {
    pub id_name: String,
    pub is_open: bool,
    pub room_a: Entity,
    pub room_b: Entity,
}

#[derive(Component, Debug)]
pub struct ThermalThreshold {
    pub critical: f32,
    pub timer: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulnerabilityType {
    PowerShortage,
    DoorMalfunction,
    HullBreach,
}

#[derive(Component, Debug, Clone)]
pub struct Vulnerabilities(pub Vec<VulnerabilityType>);

#[derive(Component, Debug)]
pub struct RepairTask(pub Timer);

#[derive(Component, Debug, Clone, Copy)]
pub struct Oxygen(pub f32);

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct OxygenDelta(pub f32);

#[derive(Component, Debug, Clone, Copy)]
pub struct LifeSupport {
    pub output_rate: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineType {
    Reactor,
    LifeSupport,
    Server,
    Cooler,
}

impl MachineType {
    pub fn short_code(&self) -> &'static str {
        match self {
            MachineType::Reactor => "RCT",
            MachineType::LifeSupport => "LFS",
            MachineType::Server => "SRV",
            MachineType::Cooler => "COL",
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Machine {
    pub id_name: String,
    pub machine_type: MachineType,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct LocatedIn(pub Entity);
