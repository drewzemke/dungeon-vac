use bevy::prelude::*;

use crate::{core::rule::Rule, game::map::MapSetup, messages::LoadLevel};

#[derive(Default, Resource)]
pub struct Solution {
    rules: Vec<Rule>,
}

impl Solution {
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, idx: usize) {
        self.rules.remove(idx);
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

fn on_load_level(mut solution: ResMut<Solution>) {
    solution.reset();
}

pub struct SolutionPlugin;

impl Plugin for SolutionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Solution>().add_systems(
            Update,
            on_load_level
                .in_set(MapSetup)
                .run_if(on_message::<LoadLevel>),
        );
    }
}
