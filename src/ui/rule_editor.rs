use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{
    core::rule::Rule,
    game::{level::LevelProgression, simulation::Simulation},
    messages::{LevelComplete, NextLevel, ResetLevel},
};

/// UI state for rule creation
#[derive(Default, Resource)]
pub struct UiState {
    selected_sensor: usize,
    selected_command: usize,

    // FIXME: this should be part of simulator state
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
                    // FIXME: disable the button if sensor+comman is not selected
                    if let Some(sensor) = sensors.get(state.selected_sensor)
                        && let Some(command) = commands.get(state.selected_command)
                    {
                        rules.push(Rule::new(*sensor, *command));
                    }
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

pub fn on_level_complete(mut reader: MessageReader<LevelComplete>, mut state: ResMut<UiState>) {
    for _ in reader.read() {
        state.level_complete = true;
    }
}
