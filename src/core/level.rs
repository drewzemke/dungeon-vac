use crate::core::{command::Command, map::Map, sensor::Sensor};

#[derive(Debug)]
pub struct Level {
    /// the layout and positions of things in this level
    map: Map,

    /// the sensors available in this level
    sensors: Vec<Sensor>,

    /// the commands available in this level
    commands: Vec<Command>,
}

impl Level {
    pub fn new(map: Map, sensors: Vec<Sensor>, commands: Vec<Command>) -> Self {
        Self {
            map,
            sensors,
            commands,
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn sensors(&self) -> &[Sensor] {
        &self.sensors
    }
}
