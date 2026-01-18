use bevy::prelude::*;

use crate::{
    core::{level::Level, map::Map},
    game::{map::MapSetup, messages::LoadLevel},
};

#[derive(Resource)]
pub struct LevelProgression {
    levels: Vec<Level>,
    current_idx: usize,
}

impl LevelProgression {
    pub fn default_levels() -> Self {
        let levels = vec![level_1_basics()];
        Self {
            levels,
            current_idx: 0,
        }
    }

    pub fn current(&self) -> &Level {
        &self.levels[self.current_idx]
    }

    pub fn set_current(&mut self, idx: usize) {
        self.current_idx = idx;
    }
}

fn init_level(mut writer: MessageWriter<LoadLevel>) {
    writer.write(LoadLevel(0));
}

fn load_level(mut reader: MessageReader<LoadLevel>, mut levels: ResMut<LevelProgression>) {
    if let Some(LoadLevel(idx)) = reader.read().last() {
        levels.set_current(*idx);
    }
}

pub struct DefaultLevelsPlugin;

impl Plugin for DefaultLevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelProgression::default_levels())
            .add_systems(Startup, init_level)
            .add_systems(
                Update,
                load_level.before(MapSetup).run_if(on_message::<LoadLevel>),
            );
    }
}

// --- levels ---

fn level_1_basics() -> Level {
    let map = Map::parse(
        r"
########
#S....E#
########
",
    )
    .unwrap();

    Level::new(map, vec![], vec![])
}

// TODO: add more levels

// const MAP_STR: &str = r"#######
// #S..###
// #.#.###
// #.#...#
// #.#.#.#
// #.#E..#
// #.###.#
// #.....#
// #######
// ";
