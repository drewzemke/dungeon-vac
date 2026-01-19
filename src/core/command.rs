#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Movement(MovementCommand),
    Cleaning(CleaningCommand),
}

// MOVEMENT

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

// CLEANING

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CleaningCommand {
    StartCleaning,
    StopCleaning,
}

impl From<CleaningCommand> for Command {
    fn from(command: CleaningCommand) -> Self {
        Self::Cleaning(command)
    }
}

/// Represents the full set of commands for the roomba in a single tick
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CommandSet {
    movement: Option<MovementCommand>,
    cleaning: Option<CleaningCommand>,
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
            Command::Cleaning(cmd) => {
                if self.cleaning.is_none() {
                    self.cleaning = Some(cmd);
                }
            }
        }
    }

    pub fn movement(&self) -> Option<MovementCommand> {
        self.movement
    }

    pub fn cleaning(&self) -> Option<CleaningCommand> {
        self.cleaning
    }

    pub fn clear_movement(&mut self) {
        self.movement = None;
    }
}
