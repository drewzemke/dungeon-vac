#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Movement(MovementCommand),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovementCommand {
    TurnRight,
    TurnLeft,
}

impl From<MovementCommand> for Command {
    fn from(command: MovementCommand) -> Self {
        Self::Movement(command)
    }
}

/// Represents the full set of commands for the roomba in a single tick
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CommandSet {
    movement: Option<MovementCommand>,
    // vacuum: Option<...>
}

impl CommandSet {
    /// Adds a command to the set. If a command in the same category (eg. movement, vacuum)
    /// is already set, the new command is ignored.
    pub fn add(&mut self, command: impl Into<Command>) {
        match command.into() {
            Command::Movement(cmd) => {
                if self.movement.is_none() {
                    self.movement = Some(cmd);
                }
            }
        }
    }

    pub fn movement(&self) -> Option<MovementCommand> {
        self.movement
    }

    pub fn clear_movement(&mut self) {
        self.movement = None;
    }
}
