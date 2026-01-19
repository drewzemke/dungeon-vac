use bevy::prelude::*;

use crate::messages::{LevelComplete, NextLevel, ResetLevel, StartSimulation};

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

fn on_reset_level(mut sim: ResMut<Simulation>) {
    sim.reset();
}

fn on_next_level(mut sim: ResMut<Simulation>) {
    sim.reset();
}

fn on_level_complete(mut sim: ResMut<Simulation>) {
    sim.stop();
}

fn on_start_sim(mut sim: ResMut<Simulation>) {
    sim.start();
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Simulation::default()).add_systems(
            Update,
            (
                on_level_complete.run_if(on_message::<LevelComplete>),
                on_next_level.run_if(on_message::<NextLevel>),
                on_reset_level.run_if(on_message::<ResetLevel>),
                on_start_sim.run_if(on_message::<StartSimulation>),
            ),
        );
    }
}
