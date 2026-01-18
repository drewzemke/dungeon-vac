use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{
    core::rule::Rule,
    game::{
        level::LevelProgression,
        messages::{LevelComplete, ResetLevel},
        simulation::Simulation,
    },
};

/// UI state for rule creation
#[derive(Default, Resource)]
pub struct UiState {
    selected_sensor: usize,
    selected_command: usize,

    level_complete: bool,
}

// FIXME: this isn't the right place for this
#[derive(Default, Resource, Deref, DerefMut)]
pub struct Rules(pub Vec<Rule>);

pub fn rule_editor_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut rules: ResMut<Rules>,
    mut sim: ResMut<Simulation>,
    levels: Res<LevelProgression>,
    mut writer: MessageWriter<ResetLevel>,
) {
    let level = levels.current();
    let sensors = level.sensors();
    let commands = level.commands();

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let running = sim.is_running();
    let can_edit = !sim.has_started();

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
                    writer.write(ResetLevel);
                }
            });

            ui.separator();

            ui.label("Create Rule:");
            ui.add_space(8.0);

            ui.add_enabled_ui(can_edit, |ui| {
                let selected_sensor = sensors
                    .get(state.selected_sensor)
                    .map(|s| s.to_string())
                    .unwrap_or("Select".into());
                egui::ComboBox::from_label("Sensor")
                    .selected_text(selected_sensor)
                    .show_ui(ui, |ui| {
                        for (i, sensor) in sensors.iter().enumerate() {
                            ui.selectable_value(&mut state.selected_sensor, i, *sensor);
                        }
                    });

                let selected_command = commands
                    .get(state.selected_command)
                    .map(|c| c.to_string())
                    .unwrap_or("Select".into());
                egui::ComboBox::from_label("Command")
                    .selected_text(selected_command)
                    .show_ui(ui, |ui| {
                        for (i, command) in commands.iter().enumerate() {
                            ui.selectable_value(&mut state.selected_command, i, *command);
                        }
                    });

                ui.add_space(8.0);
                if ui.button("Add Rule").clicked() {
                    let sensor = sensors[state.selected_sensor];
                    let command = commands[state.selected_command];
                    rules.push(Rule::new(sensor, command));
                }
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

            if state.level_complete {
                ui.separator();
                ui.label("Level Complete!");
            }
        });
}

pub fn on_level_complete(mut reader: MessageReader<LevelComplete>, mut state: ResMut<UiState>) {
    for _ in reader.read() {
        state.level_complete = true;
    }
}
