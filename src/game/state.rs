use bevy::math::IVec2;

use crate::game::{action::Action, dir::Dir, event::Event, level::Level, rule::Rule};

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

    /// returns the actions that were taken
    pub fn tick(&mut self, level: &Level, rules: &[Rule]) -> Effect {
        let events = self.evaluate_events(level);
        let actions = Rule::compute_actions(rules, &events);

        // HACK: until we expand to have categories, there will only ever
        // be one action
        assert_eq!(actions.len(), 1);
        self.apply_action(actions[0], level)
    }

    fn apply_action(&mut self, action: Action, level: &Level) -> Effect {
        match action {
            Action::MoveForward => {
                let orig_pos = self.vac_pos;
                // check for a wall collision
                let dest = orig_pos + self.vac_dir.to_vec();

                if level.has_space(dest) {
                    self.vac_pos = dest;
                    self.hit_wall_last_tick = false;
                    Effect::Moved {
                        from: orig_pos,
                        to: self.vac_pos,
                    }
                } else {
                    self.hit_wall_last_tick = true;
                    Effect::BumpedWall
                }
            }
            Action::TurnRight => {
                let orig_dir = self.vac_dir;
                self.vac_dir = orig_dir.rotate_cw();
                Effect::Rotated {
                    from: orig_dir,
                    to: self.vac_dir,
                }
            }
            Action::TurnLeft => {
                let orig_dir = self.vac_dir;
                self.vac_dir = orig_dir.rotate_ccw();
                Effect::Rotated {
                    from: orig_dir,
                    to: self.vac_dir,
                }
            }
        }
    }

    fn evaluate_events(&self, level: &Level) -> Vec<Event> {
        let mut events = Vec::new();

        let left = self.vac_pos + self.vac_dir.rotate_ccw().to_vec();
        if level.has_space(left) {
            events.push(Event::SpaceLeft);
        }

        let right = self.vac_pos + self.vac_dir.rotate_cw().to_vec();
        if level.has_space(right) {
            events.push(Event::SpaceRight);
        }

        if self.hit_wall_last_tick {
            events.push(Event::HitWall);
        }

        events
    }
}

#[cfg(test)]
mod tests {
    use crate::game::rule::Rule;

    use super::*;

    #[test]
    fn test_evaluate_events() {
        let level = Level::parse(Level::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        // given that setup, there should be space on the left but not on the right
        let events = state.evaluate_events(&level);
        assert!(events.contains(&Event::SpaceLeft));
        assert!(!events.contains(&Event::SpaceRight));
        assert!(!events.contains(&Event::HitWall));

        state.hit_wall_last_tick = true;
        let events = state.evaluate_events(&level);
        assert!(events.contains(&Event::HitWall));
    }

    #[test]
    fn test_apply_actions_no_walls() {
        let level = Level::parse(Level::EMPTY_3X3).unwrap();
        let mut state = State::new((0, 0), Dir::East);

        let effect = state.apply_action(Action::MoveForward, &level);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::East);
        assert_eq!(
            effect,
            Effect::Moved {
                from: (0, 0).into(),
                to: (1, 0).into()
            }
        );

        let effect = state.apply_action(Action::TurnRight, &level);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::South);
        assert_eq!(
            effect,
            Effect::Rotated {
                from: Dir::East,
                to: Dir::South
            }
        );

        let effect = state.apply_action(Action::TurnLeft, &level);
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
    fn test_apply_actions_with_walls() {
        let level = Level::parse(Level::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        // there's one space to move to in this direction before we hit a wall
        let effect = state.apply_action(Action::MoveForward, &level);
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
        let effect = state.apply_action(Action::MoveForward, &level);
        assert_eq!(state.vac_pos, (2, 1).into());
        assert!(state.hit_wall_last_tick);
        assert_eq!(effect, Effect::BumpedWall);
    }

    #[test]
    fn test_tick() {
        let level = Level::parse(Level::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        let rules = [
            Rule::new(Event::SpaceRight, Action::TurnRight),
            Rule::new(Event::SpaceLeft, Action::TurnLeft),
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
