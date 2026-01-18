use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use dungeon_vac::{
    core::{command::Command, rule::Rule, sensor::Sensor},
    game::{
        level::DefaultLevelsPlugin, map::MapPlugin, messages::MessagesPlugin,
        simulation::SimulationPlugin, vac::VacPlugin,
    },
    ui::{
        camera::CameraPlugin,
        grid::GridPlugin,
        rule_editor::{Rules, UiState, on_level_complete, rule_editor_ui},
    },
};

const RULES: [Rule; 2] = [
    Rule::new(Sensor::SpaceRight, Command::TurnRight),
    Rule::new(Sensor::HitWall, Command::TurnLeft),
];

fn main() {
    // let first_level = Level::new(
    //     Map::parse(MAP_STR).unwrap(),
    //     Vec::from([Sensor::HitWall, Sensor::SpaceLeft, Sensor::SpaceRight]),
    //     Vec::from([Command::TurnRight, Command::TurnLeft]),
    // );

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
        .add_plugins(DefaultLevelsPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(VacPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(MessagesPlugin)
        .insert_resource(Rules(Vec::from(RULES)))
        .init_resource::<UiState>()
        // FIXME: extract to UI plugin
        .add_systems(EguiPrimaryContextPass, rule_editor_ui)
        .add_systems(Update, on_level_complete)
        //
        .run();
}
