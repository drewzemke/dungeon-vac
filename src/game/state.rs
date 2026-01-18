use bevy::prelude::*;

use crate::core::state::State as CoreState;

#[derive(Component, Deref, DerefMut)]
pub struct State(pub CoreState);
