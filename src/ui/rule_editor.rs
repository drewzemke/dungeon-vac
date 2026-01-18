use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::{
    core::rule::Rule,
    game::{level::LevelProgression, simulation::Simulation},
    messages::{LevelComplete, NextLevel, ResetLevel},
};

/// UI state for rule creation
#[derive(Default, Resource)]
struct UiState {
    selected_sensor: Option<usize>,
    selected_command: Option<usize>,

    // FIXME: this should be part of simulator state
    level_complete: bool,
}

// FIXME: this isn't the right place for this
// extract to a `solution` resource?
#[derive(Default, Resource, Deref, DerefMut)]
pub struct Rules(pub Vec<Rule>);

fn rule_editor_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut rules: ResMut<Rules>,
    mut sim: ResMut<Simulation>,
    levels: Res<LevelProgression>,
    mut reset: MessageWriter<ResetLevel>,
    mut next_level: MessageWriter<NextLevel>,
) {
    let level = levels.current();
    let sensors = level.sensors();
    let commands = level.commands();

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let running = sim.is_running();
    let can_edit = !sim.has_started();
    let level_complete = state.level_complete;

    egui::SidePanel::left("rule_editor")
        .resizable(false)
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_enabled_ui(!running, |ui| {
                    if ui.button("Start").clicked() {
                        // TODO: turn this into an event
                        sim.start();
                    }
                });

                ui.add_enabled_ui(running, |ui| {
                    if ui.button("Stop").clicked() {
                        // TODO: turn this into an event
                        sim.stop();
                    }
                });

                if ui.button("Reset").clicked() {
                    reset.write(ResetLevel);
                }
            });

            ui.separator();

            ui.label("Create Rule:");
            ui.add_space(8.0);

            let selected_sensor = state.selected_sensor.and_then(|idx| sensors.get(idx));
            let selected_command = state.selected_command.and_then(|idx| commands.get(idx));

            ui.add_enabled_ui(can_edit, |ui| {
                let selected_sensor_str = selected_sensor
                    .map(|s| s.to_string())
                    .unwrap_or("Select".into());
                egui::ComboBox::from_label("Sensor")
                    .selected_text(selected_sensor_str)
                    .show_ui(ui, |ui| {
                        for (i, sensor) in sensors.iter().enumerate() {
                            ui.selectable_value(&mut state.selected_sensor, Some(i), *sensor);
                        }
                    });

                let selected_command_str = selected_command
                    .map(|s| s.to_string())
                    .unwrap_or("Select".into());
                egui::ComboBox::from_label("Command")
                    .selected_text(selected_command_str)
                    .show_ui(ui, |ui| {
                        for (i, command) in commands.iter().enumerate() {
                            ui.selectable_value(&mut state.selected_command, Some(i), *command);
                        }
                    });

                ui.add_space(8.0);
                ui.add_enabled_ui(
                    selected_sensor.is_some() && selected_command.is_some(),
                    |ui| {
                        if ui.button("Add Rule").clicked()
                            && let Some(sensor) = selected_sensor
                            && let Some(command) = selected_command
                        {
                            rules.push(Rule::new(*sensor, *command));
                            state.selected_sensor = None;
                            state.selected_command = None;
                        }
                    },
                );
            });

            ui.separator();

            ui.label("Rules:");
            ui.add_space(8.0);

            let mut remove_idx = None;
            for (idx, rule) in rules.0.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{} {}", rule.sensor(), rule.command()));
                    ui.add_enabled_ui(can_edit, |ui| {
                        if ui.button("X").clicked() {
                            remove_idx = Some(idx);
                        }
                    });
                });
            }

            if let Some(idx) = remove_idx {
                rules.remove(idx);
            }

            if level_complete {
                ui.separator();
                ui.label("Level Complete!");
                if ui.button("Next Level").clicked() {
                    state.level_complete = false;
                    next_level.write(NextLevel);
                }
            }
        });
}

fn on_level_complete(mut reader: MessageReader<LevelComplete>, mut state: ResMut<UiState>) {
    for _ in reader.read() {
        state.level_complete = true;
    }
}

pub struct RuleEditorPlugin;

impl Plugin for RuleEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_systems(EguiPrimaryContextPass, rule_editor_ui)
            .add_systems(Update, on_level_complete);
    }
}
