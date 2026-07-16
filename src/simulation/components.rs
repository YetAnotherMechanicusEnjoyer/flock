use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Room {
    pub name: &'static str,
    pub volume: f32,
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
