use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use dungeon_vac::{
    game::{
        level::DefaultLevelsPlugin, map::MapPlugin, simulation::SimulationPlugin, vac::VacPlugin,
    },
    messages::MessagesPlugin,
    ui::{
        camera::CameraPlugin,
        grid::GridPlugin,
        rule_editor::{RuleEditorPlugin, Rules},
    },
};

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
        .add_plugins(CameraPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(DefaultLevelsPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(VacPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(MessagesPlugin)
        .add_plugins(RuleEditorPlugin)
        .insert_resource(Rules(Vec::new()))
        .run();
}
