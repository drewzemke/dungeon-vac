use bevy::prelude::*;

#[derive(Message)]
pub struct LevelComplete;

#[derive(Message)]
pub struct LoadLevel(pub usize);
