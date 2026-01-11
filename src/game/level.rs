use bevy::prelude::*;

use crate::core::level::Level;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CurrentLevel(Level);

impl CurrentLevel {
    pub fn new(level: Level) -> Self {
        Self(level)
    }
}
