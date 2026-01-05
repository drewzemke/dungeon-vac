use bevy::math::IVec2;

use super::{action::Action, dir::Dir, level::Level};

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
                let dest = self.vac_pos + IVec2::from(self.vac_dir);

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
