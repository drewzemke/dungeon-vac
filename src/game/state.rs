use bevy::math::IVec2;

use crate::game::{command::Command, dir::Dir, level::Level, rule::Rule, sensor::Sensor};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effect {
    Moved { from: IVec2, to: IVec2 },
    BumpedWall,
    Rotated { from: Dir, to: Dir },
}

pub struct State {
    vac_pos: IVec2,
    vac_dir: Dir,

    hit_wall_last_tick: bool,
}

impl State {
    pub fn new(vac_pos: impl Into<IVec2>, vac_dir: Dir) -> Self {
        Self {
            vac_pos: vac_pos.into(),
            vac_dir,

            hit_wall_last_tick: false,
        }
    }

    pub fn tick(&mut self, level: &Level, rules: &[Rule]) -> Effect {
        let sensors = self.evaluate_sensors(level);

        self.reset_flags();

        let commands = Rule::compute_commands(rules, &sensors);

        // HACK: until we expand to have categories, there will only ever
        // be one command
        assert_eq!(commands.len(), 1);
        self.apply_command(commands[0], level)
    }

    fn reset_flags(&mut self) {
        self.hit_wall_last_tick = false;
    }

    fn apply_command(&mut self, command: Command, level: &Level) -> Effect {
        match command {
            Command::MoveForward => {
                let orig_pos = self.vac_pos;
                // check for a wall collision
                let dest = orig_pos + self.vac_dir.to_ivec();

                if level.has_space(dest) {
                    self.vac_pos = dest;
                    Effect::Moved {
                        from: orig_pos,
                        to: self.vac_pos,
                    }
                } else {
                    self.hit_wall_last_tick = true;
                    Effect::BumpedWall
                }
            }
            Command::TurnRight => {
                let orig_dir = self.vac_dir;
                self.vac_dir = orig_dir.rotate_cw();
                Effect::Rotated {
                    from: orig_dir,
                    to: self.vac_dir,
                }
            }
            Command::TurnLeft => {
                let orig_dir = self.vac_dir;
                self.vac_dir = orig_dir.rotate_ccw();
                Effect::Rotated {
                    from: orig_dir,
                    to: self.vac_dir,
                }
            }
        }
    }

    fn evaluate_sensors(&self, level: &Level) -> Vec<Sensor> {
        let mut sensors = Vec::new();

        let left = self.vac_pos + self.vac_dir.rotate_ccw().to_ivec();
        if level.has_space(left) {
            sensors.push(Sensor::SpaceLeft);
        }

        let right = self.vac_pos + self.vac_dir.rotate_cw().to_ivec();
        if level.has_space(right) {
            sensors.push(Sensor::SpaceRight);
        }

        if self.hit_wall_last_tick {
            sensors.push(Sensor::HitWall);
        }

        sensors
    }

    pub fn vac_pos(&self) -> IVec2 {
        self.vac_pos
    }

    pub fn vac_dir(&self) -> Dir {
        self.vac_dir
    }
}

#[cfg(test)]
mod tests {
    use crate::game::rule::Rule;

    use super::*;

    #[test]
    fn test_evaluate_sensors() {
        let level = Level::parse(Level::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        // given that setup, there should be space on the left but not on the right
        let sensors = state.evaluate_sensors(&level);
        assert!(sensors.contains(&Sensor::SpaceLeft));
        assert!(!sensors.contains(&Sensor::SpaceRight));
        assert!(!sensors.contains(&Sensor::HitWall));

        state.hit_wall_last_tick = true;
        let sensors = state.evaluate_sensors(&level);
        assert!(sensors.contains(&Sensor::HitWall));
    }

    #[test]
    fn test_apply_commands_no_walls() {
        let level = Level::parse(Level::EMPTY_3X3).unwrap();
        let mut state = State::new((0, 0), Dir::East);

        let effect = state.apply_command(Command::MoveForward, &level);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::East);
        assert_eq!(
            effect,
            Effect::Moved {
                from: (0, 0).into(),
                to: (1, 0).into()
            }
        );

        let effect = state.apply_command(Command::TurnRight, &level);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::South);
        assert_eq!(
            effect,
            Effect::Rotated {
                from: Dir::East,
                to: Dir::South
            }
        );

        let effect = state.apply_command(Command::TurnLeft, &level);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::East);
        assert_eq!(
            effect,
            Effect::Rotated {
                from: Dir::South,
                to: Dir::East
            }
        );
    }

    #[test]
    fn test_apply_commands_with_walls() {
        let level = Level::parse(Level::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        // there's one space to move to in this direction before we hit a wall
        let effect = state.apply_command(Command::MoveForward, &level);
        assert_eq!(state.vac_pos, (2, 1).into());
        assert!(!state.hit_wall_last_tick);
        assert_eq!(
            effect,
            Effect::Moved {
                from: (1, 1).into(),
                to: (2, 1).into()
            }
        );

        // shouldn't be able to move forward again
        let effect = state.apply_command(Command::MoveForward, &level);
        assert_eq!(state.vac_pos, (2, 1).into());
        assert!(state.hit_wall_last_tick);
        assert_eq!(effect, Effect::BumpedWall);
    }

    #[test]
    fn test_tick() {
        let level = Level::parse(Level::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        let rules = [
            Rule::new(Sensor::SpaceRight, Command::TurnRight),
            Rule::new(Sensor::SpaceLeft, Command::TurnLeft),
        ];

        // there's space on the left but not the right,
        // so we should turn left
        let effect = state.tick(&level, &rules);
        assert_eq!(state.vac_dir, Dir::North);
        assert_eq!(
            effect,
            Effect::Rotated {
                from: Dir::East,
                to: Dir::North
            }
        );
    }
}
