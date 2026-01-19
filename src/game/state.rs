use bevy::prelude::*;

use crate::core::state::State as CoreState;

#[derive(Component, Deref, DerefMut, Default)]
pub struct State(pub Option<CoreState>);

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(state: CoreState) -> Self {
        Self(Some(state))
    }
}
