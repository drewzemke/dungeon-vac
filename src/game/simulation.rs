use bevy::prelude::*;

use crate::messages::{LevelComplete, ResetLevel};

#[derive(Default, Resource)]
pub struct Simulation {
    started: bool,
    running: bool,
}

impl Simulation {
    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn has_started(&self) -> bool {
        self.started
    }

    pub fn start(&mut self) {
        self.started = true;
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self) {
        self.started = false;
        self.running = false;
    }
}

// TODO: use run_if to schedule
fn on_reset_level(mut sim: ResMut<Simulation>, mut reader: MessageReader<ResetLevel>) {
    if reader.read().count() > 0 {
        sim.reset();
    }
}

// TODO: use run_if to schedule
fn on_level_complete(mut sim: ResMut<Simulation>, mut reader: MessageReader<LevelComplete>) {
    if reader.read().count() > 0 {
        sim.stop();
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Simulation::default())
            .add_systems(Update, (on_reset_level, on_level_complete));
    }
}
