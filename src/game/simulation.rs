use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Simulation {
    running: bool,
}

impl Simulation {
    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
