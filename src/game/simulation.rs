use bevy::prelude::*;

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
}
