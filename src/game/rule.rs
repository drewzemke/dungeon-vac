use crate::game::{command::Command, sensor::Sensor};

#[derive(Debug)]
pub struct Rule {
    sensor: Sensor,
    command: Command,
}

impl Rule {
    pub const fn new(sensor: Sensor, command: Command) -> Self {
        Self { sensor, command }
    }

    pub fn compute_commands(rules: &[Rule], sensor: &[Sensor]) -> Vec<Command> {
        let mut commands = Vec::new();

        for rule in rules {
            for sensor in sensor {
                // check for rule match
                if rule.sensor != *sensor {
                    continue;
                }

                // filter out commands that already have a member of their category in
                // the output commands
                // NOTE: this is trivial at the moment as there's only one category
                let category_already_represented = !commands.is_empty();
                if !category_already_represented {
                    commands.push(rule.command);
                }
            }
        }

        // if no movement command was fired, fall back to moving forward
        // NOTE: update this logic when we add categories
        if commands.is_empty() {
            commands.push(Command::MoveForward);
        }

        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_commands_default_move() {
        let rules = [Rule::new(Sensor::HitWall, Command::TurnRight)];
        let sensor = [Sensor::SpaceLeft];

        let commands = Rule::compute_commands(&rules, &sensor);
        assert_eq!(commands, vec![Command::MoveForward]);
    }

    #[test]
    fn compute_commands_single() {
        let rules = [Rule::new(Sensor::HitWall, Command::TurnRight)];
        let sensors = [Sensor::HitWall];

        let commands = Rule::compute_commands(&rules, &sensors);
        assert_eq!(commands, vec![Command::TurnRight]);
    }

    #[test]
    fn compute_commands_same_category() {
        let rules = [
            Rule::new(Sensor::HitWall, Command::TurnRight),
            Rule::new(Sensor::HitWall, Command::TurnLeft),
        ];
        let sensors = [Sensor::HitWall];

        // only the first matching rule should trigger
        let commands = Rule::compute_commands(&rules, &sensors);
        assert_eq!(commands, vec![Command::TurnRight]);
    }
}
