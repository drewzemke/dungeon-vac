#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    TurnRight,
    TurnLeft,

    // FIXME: this shouldn't be a command as it isn't
    // something the player can control
    MoveForward,
}
