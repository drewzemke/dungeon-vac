use bevy_egui::egui::WidgetText;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    TurnRight,
    TurnLeft,

    // FIXME: this shouldn't be a command as it isn't
    // something the player can control
    MoveForward,
}

// FIXME: should this live somewhere else? it isn't purely about game logic
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

impl From<Command> for WidgetText {
    fn from(val: Command) -> Self {
        String::from(val).into()
    }
}
