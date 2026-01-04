use bevy::math::IVec2;

use super::{action::Action, dir::Dir};

pub struct State {
    vac_pos: IVec2,
    vac_dir: Dir,
}

impl State {
    pub fn new(vac_pos: impl Into<IVec2>, vac_dir: Dir) -> Self {
        Self {
            vac_pos: vac_pos.into(),
            vac_dir,
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::MoveForward => {
                self.vac_pos += IVec2::from(self.vac_dir);
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
    fn apply_actions() {
        let mut state = State::new((0, 0), Dir::East);

        state.apply_action(Action::MoveForward);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::East);

        state.apply_action(Action::TurnRight);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::South);

        state.apply_action(Action::TurnLeft);
        assert_eq!(state.vac_pos, (1, 0).into());
        assert_eq!(state.vac_dir, Dir::East);
    }
}
