use bevy::prelude::*;

#[derive(Message)]
pub struct TrashCollected(pub IVec2);

#[derive(Message)]
pub struct LevelComplete;

#[derive(Message)]
pub struct LoadLevel(pub usize);
