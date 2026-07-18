use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::{simulation::components::*, terminal::components::PrintToTerminal};

#[derive(Message, Debug)]
pub struct CriticalTemperatureEvent {
    pub entity: Entity,
    pub room_name: String,
}

pub fn detect_critical_temperatures(
    time: Res<Time<Fixed>>,
    mut query: Query<(Entity, &Room, &Temperature, &mut ThermalThreshold)>,
    mut ev_critical: MessageWriter<CriticalTemperatureEvent>,
) {
    for (entity, room, temp, mut threshold) in query.iter_mut() {
        threshold.timer.tick(time.delta());

        if temp.current >= threshold.critical && threshold.timer.just_finished() {
            ev_critical.write(CriticalTemperatureEvent {
                entity,
                room_name: room.name.to_string(),
            });
        }
    }
}

pub fn resolve_critical_events(
    mut ev_critical: MessageReader<CriticalTemperatureEvent>,
    mut room_query: Query<(&mut PowerState, &mut HullIntegrity, &Vulnerabilities)>,
    mut door_query: Query<&mut Door>,
    mut printer: MessageWriter<PrintToTerminal>,
) {
    let mut rng = rand::rng();

    for ev in ev_critical.read() {
        if let Ok((mut power, mut hull, vulns)) = room_query.get_mut(ev.entity) {
            if vulns.0.is_empty() {
                continue;
            }

            let chosen = vulns.0.choose(&mut rng).unwrap();

            match chosen {
                VulnerabilityType::PowerShortage => {
                    if *power != PowerState::Offline {
                        *power = PowerState::Offline;
                        printer.write(PrintToTerminal(format!(
                            "⚠ DISASTER: Power shorted in {} due to extreme heat!",
                            ev.room_name
                        )));
                    }
                }
                VulnerabilityType::DoorMalfunction => {
                    let mut jammed = false;
                    for mut door in door_query.iter_mut() {
                        if door.room_a == ev.entity || door.room_b == ev.entity {
                            door.is_open = !door.is_open;
                            printer.write(PrintToTerminal(format!(
                                "⚠ DISASTER: Thermal expansion jammed door {} in {}!",
                                door.id_name, ev.room_name
                            )));
                            jammed = true;
                            break;
                        }
                    }
                    if !jammed {
                        printer.write(PrintToTerminal(format!(
                            "⚠ WARNING: Extreme heat detected in {}!",
                            ev.room_name
                        )));
                    }
                }
                VulnerabilityType::HullBreach => {
                    hull.0 = (hull.0 - 15.0).max(0.0);
                    printer.write(PrintToTerminal(format!(
                        "⚠ CRITICAL: Hull integrity failing in {} ({}%)!",
                        ev.room_name, hull.0
                    )));
                }
            }
        }
    }
}
