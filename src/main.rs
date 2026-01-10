use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use dungeon_vac::{
    core::{command::Command, rule::Rule, sensor::Sensor},
    game::{map::MapPlugin, vac::VacPlugin},
    ui::{
        grid::GridPlugin,
        rule_editor::{RuleEditor, Rules, rule_editor_ui},
    },
};

const RULES: [Rule; 2] = [
    Rule::new(Sensor::SpaceRight, Command::TurnRight),
    Rule::new(Sensor::HitWall, Command::TurnLeft),
];

fn main() {
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
        .add_plugins(MapPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(VacPlugin)
        .insert_resource(Rules(Vec::from(RULES)))
        .init_resource::<RuleEditor>()
        .add_systems(Startup, setup_camera)
        .add_systems(EguiPrimaryContextPass, rule_editor_ui)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
