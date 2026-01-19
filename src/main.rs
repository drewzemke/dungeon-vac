use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use clap::Parser;

use dungeon_vac::{
    game::{
        level::DefaultLevelsPlugin, map::MapPlugin, simulation::SimulationPlugin,
        solution::SolutionPlugin, vac::VacPlugin,
    },
    messages::MessagesPlugin,
    ui::{camera::CameraPlugin, grid::GridPlugin, rule_editor::RuleEditorPlugin},
};

#[derive(Parser)]
#[command(name = "dungeon-vac")]
struct Args {
    /// Starting level (1-indexed)
    #[arg(short, long)]
    level: Option<usize>,
}

fn main() {
    let args = Args::parse();
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
        .add_plugins(DefaultLevelsPlugin {
            starting_level: args.level.map(|l| l.saturating_sub(1)).unwrap_or(0),
        })
        .add_plugins(MapPlugin)
        .add_plugins(VacPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(MessagesPlugin)
        .add_plugins(RuleEditorPlugin)
        .add_plugins(SolutionPlugin)
        .run();
}
