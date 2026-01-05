use bevy::math::IVec2;

use super::{action::Action, dir::Dir, event::Event, level::Level};

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

    pub fn apply_action(&mut self, action: Action, level: &Level) {
        match action {
            Action::MoveForward => {
                // check for a wall collision
                let dest = self.vac_pos + self.vac_dir.to_vec();

                if level.has_space(dest) {
                    self.vac_pos = dest;
                    self.hit_wall_last_tick = false;
                } else {
                    self.hit_wall_last_tick = true;
                }
            }
            Action::TurnRight => {
                self.vac_dir = self.vac_dir.rotate_cw();
            }
            Action::TurnLeft => {
                self.vac_dir = self.vac_dir.rotate_ccw();
            }
        }
    }

    pub fn evaluate_events(&self, level: &Level) -> Vec<Event> {
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
    use super::*;

    #[test]
    fn evaluate_events() {
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
    fn apply_actions_no_walls() {
        let level = Level::parse(Level::EMPTY_3X3).unwrap();
        let mut state = State::new((0, 0), Dir::East);

        state.apply_action(Action::MoveForward, &level);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::East);

        state.apply_action(Action::TurnRight, &level);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::South);

        state.apply_action(Action::TurnLeft, &level);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::East);
    }

    #[test]
    fn apply_actions_with_walls() {
        let level = Level::parse(Level::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        // there's one space to move to in this direction before we hit a wall
        state.apply_action(Action::MoveForward, &level);
        assert_eq!(state.vac_pos, (2, 1).into());
        assert!(!state.hit_wall_last_tick);

        // shouldn't be able to move forward again
        state.apply_action(Action::MoveForward, &level);
        assert_eq!(state.vac_pos, (2, 1).into());
        assert!(state.hit_wall_last_tick);
    }
}
