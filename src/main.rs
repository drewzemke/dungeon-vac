use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use dungeon_vac::{
    game::{
        level::DefaultLevelsPlugin, map::MapPlugin, simulation::SimulationPlugin, vac::VacPlugin,
    },
    messages::MessagesPlugin,
    ui::{
        camera::CameraPlugin,
        grid::GridPlugin,
        rule_editor::{Rules, UiState, on_level_complete, rule_editor_ui},
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
        .insert_resource(Rules(Vec::new()))
        .init_resource::<UiState>()
        // FIXME: extract to UI plugin
        .add_systems(EguiPrimaryContextPass, rule_editor_ui)
        .add_systems(Update, on_level_complete)
        //
        .run();
}
