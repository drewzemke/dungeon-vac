use bevy::prelude::*;

use crate::{
    core::{command::Command, level::Level, map::Map, sensor::Sensor},
    game::map::MapSetup,
    messages::{LoadLevel, NextLevel, ResetLevel},
};

#[derive(Resource)]
pub struct LevelProgression {
    levels: Vec<Level>,
    current_idx: usize,
}

impl LevelProgression {
    pub fn default_levels() -> Self {
        let levels = vec![
            level_1_basics(),
            level_2_navigation(),
            level_5_more_navigation(),
        ];
        Self {
            levels,
            current_idx: 0,
        }
    }

    pub fn current(&self) -> &Level {
        &self.levels[self.current_idx]
    }

    fn set_current(&mut self, idx: usize) {
        self.current_idx = idx;
    }

    /// returns whether or not a new level was selected
    fn advance_level(&mut self) -> bool {
        if self.current_idx + 1 < self.levels.len() {
            self.current_idx += 1;
            true
        } else {
            false
        }
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

fn on_reset_level(mut writer: MessageWriter<LoadLevel>, levels: Res<LevelProgression>) {
    writer.write(LoadLevel(levels.current_idx));
}

fn on_next_level(mut writer: MessageWriter<LoadLevel>, mut levels: ResMut<LevelProgression>) {
    if levels.advance_level() {
        writer.write(LoadLevel(levels.current_idx));
    }
}

pub struct DefaultLevelsPlugin;

impl Plugin for DefaultLevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelProgression::default_levels())
            .add_systems(Startup, init_level)
            .add_systems(
                Update,
                (
                    load_level.before(MapSetup).run_if(on_message::<LoadLevel>),
                    on_reset_level.run_if(on_message::<ResetLevel>),
                    on_next_level.run_if(on_message::<NextLevel>),
                ),
            );
    }
}

// --- levels ---

fn level_1_basics() -> Level {
    let map = Map::parse(
        r"
########
#S..T.E#
########
",
    )
    .unwrap();

    Level::new(map, vec![], vec![])
}

fn level_2_navigation() -> Level {
    let map = Map::parse(
        r"
######
#S...#
####.#
####.#
####E#
######
",
    )
    .unwrap();

    Level::new(map, vec![Sensor::HitWall], vec![Command::TurnRight])
}

fn level_5_more_navigation() -> Level {
    let map = Map::parse(
        r"
#######
#S....#
#####.#
#.....#
#.#####
#....E#
#######
",
    )
    .unwrap();

    Level::new(
        map,
        vec![Sensor::HitWall, Sensor::SpaceRight, Sensor::SpaceLeft],
        vec![Command::TurnRight, Command::TurnLeft],
    )
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
