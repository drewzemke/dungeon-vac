use bevy::prelude::*;

#[derive(Message)]
pub struct StartSimulation;

#[derive(Message)]
pub struct NextLevel;

#[derive(Message)]
pub struct ResetLevel;
