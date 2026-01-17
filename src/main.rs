use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use dungeon_vac::{
    core::{command::Command, level::Level, map::Map, rule::Rule, sensor::Sensor},
    game::{level::CurrentLevel, map::MapPlugin, simulation::SimulationPlugin, vac::VacPlugin},
    ui::{
        camera::CameraPlugin,
        grid::GridPlugin,
        rule_editor::{RuleEditor, Rules, rule_editor_ui},
    },
};

const RULES: [Rule; 2] = [
    Rule::new(Sensor::SpaceRight, Command::TurnRight),
    Rule::new(Sensor::HitWall, Command::TurnLeft),
];

const MAP_STR: &str = r"#######
#S..###
#.#.###
#.#...#
#.#.#.#
#.#E..#
#.###.#
#.....#
#######
";

fn main() {
    let first_level = Level::new(
        Map::parse(MAP_STR).unwrap(),
        Vec::from([Sensor::HitWall, Sensor::SpaceLeft, Sensor::SpaceRight]),
        Vec::from([Command::TurnRight, Command::TurnLeft]),
    );

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dungeon Vac".to_string(),
                resolution: (900, 600).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(GridPlugin)
        //
        // HELP: do we need both of these?
        .insert_resource(CurrentLevel::new(first_level))
        .add_plugins(MapPlugin)
        //
        .add_plugins(VacPlugin)
        .add_plugins(SimulationPlugin)
        .insert_resource(Rules(Vec::from(RULES)))
        .init_resource::<RuleEditor>()
        .add_systems(EguiPrimaryContextPass, rule_editor_ui)
        .run();
}
