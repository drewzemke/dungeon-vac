use bevy_egui::egui::WidgetText;

use crate::core::{command::Command, sensor::Sensor};

// Sensor

impl From<Sensor> for String {
    fn from(val: Sensor) -> Self {
        match val {
            Sensor::HitWall => "WHEN hit wall",
            Sensor::SpaceLeft => "WHEN space left",
            Sensor::SpaceRight => "WHEN space right",
            Sensor::Start => "WHEN start",
        }
        .into()
    }
}

impl std::fmt::Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl From<Sensor> for WidgetText {
    fn from(val: Sensor) -> Self {
        String::from(val).into()
    }
}

// Command

impl From<Command> for WidgetText {
    fn from(val: Command) -> Self {
        String::from(val).into()
    }
}

impl From<Command> for String {
    fn from(val: Command) -> Self {
        match val {
            Command::TurnRight => "THEN turn right",
            Command::TurnLeft => "THEN turn left",
            Command::MoveForward => "WHEN go forward",
        }
        .into()
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}
