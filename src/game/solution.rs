use bevy::prelude::*;

use crate::core::rule::Rule;

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

pub struct SolutionPlugin;

impl Plugin for SolutionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Solution>();
    }
}
