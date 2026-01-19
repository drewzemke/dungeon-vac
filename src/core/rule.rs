use crate::core::{
    command::{Command, CommandSet},
    sensor::Sensor,
};

#[derive(Debug)]
pub struct Rule {
    sensor: Sensor,
    command: Command,
}

impl Rule {
    pub fn new(sensor: Sensor, command: impl Into<Command>) -> Self {
        Self {
            sensor,
            command: command.into(),
        }
    }

    pub fn sensor(&self) -> Sensor {
        self.sensor
    }

    pub fn command(&self) -> Command {
        self.command
    }

    pub fn compute_commands(rules: &[Rule], sensor: &[Sensor]) -> CommandSet {
        let mut commands = CommandSet::default();

        for rule in rules {
            for sensor in sensor {
                // check for rule match
                if rule.sensor != *sensor {
                    continue;
                }

                commands.add(rule.command);
            }
        }

        commands
    }
}

#[cfg(test)]
mod tests {
    use crate::core::command::MovementCommand;

    use super::*;

    #[test]
    fn compute_commands_single() {
        let rules = [Rule::new(Sensor::HitWall, MovementCommand::TurnRight)];
        let sensors = [Sensor::HitWall];

        let commands = Rule::compute_commands(&rules, &sensors);
        assert_eq!(commands.movement(), Some(MovementCommand::TurnRight));
    }

    #[test]
    fn compute_commands_same_category() {
        let rules = [
            Rule::new(Sensor::HitWall, MovementCommand::TurnRight),
            Rule::new(Sensor::HitWall, MovementCommand::TurnLeft),
        ];
        let sensors = [Sensor::HitWall];

        // only the first matching rule should trigger
        let commands = Rule::compute_commands(&rules, &sensors);
        assert_eq!(commands.movement(), Some(MovementCommand::TurnRight));
    }
}
