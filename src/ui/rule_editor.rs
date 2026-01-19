use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::{
    core::rule::Rule,
    game::{
        level::LevelProgression, simulation::Simulation, solution::Solution,
        state::State as GameState,
    },
    messages::{NextLevel, ResetLevel, StartSimulation},
};

/// UI state for rule creation
#[derive(Default, Resource)]
struct UiState {
    selected_sensor: Option<usize>,
    selected_command: Option<usize>,
}

// FIXME: too many args
#[expect(clippy::too_many_arguments)]
fn rule_editor_ui(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut solution: ResMut<Solution>,
    mut sim: ResMut<Simulation>,
    levels: Res<LevelProgression>,
    mut reset: MessageWriter<ResetLevel>,
    mut next_level: MessageWriter<NextLevel>,
    mut start_sim: MessageWriter<StartSimulation>,
    game_state: Query<&GameState>,
) {
    let level = levels.current();
    let sensors = level.sensors();
    let commands = level.commands();

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let state = game_state.single().unwrap();

    let running = sim.is_running();
    let can_edit = !sim.has_started();
    let level_complete = state.as_ref().is_some_and(|s| s.is_finished());

    egui::SidePanel::left("rule_editor")
        .resizable(false)
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_enabled_ui(!running, |ui| {
                    if ui.button("Start").clicked() {
                        start_sim.write(StartSimulation);
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

            let selected_sensor = ui_state.selected_sensor.and_then(|idx| sensors.get(idx));
            let selected_command = ui_state.selected_command.and_then(|idx| commands.get(idx));

            ui.add_enabled_ui(can_edit, |ui| {
                let selected_sensor_str = selected_sensor
                    .map(|s| s.to_string())
                    .unwrap_or("Select".into());
                egui::ComboBox::from_label("Sensor")
                    .selected_text(selected_sensor_str)
                    .show_ui(ui, |ui| {
                        for (i, sensor) in sensors.iter().enumerate() {
                            ui.selectable_value(&mut ui_state.selected_sensor, Some(i), *sensor);
                        }
                    });

                let selected_command_str = selected_command
                    .map(|s| s.to_string())
                    .unwrap_or("Select".into());
                egui::ComboBox::from_label("Command")
                    .selected_text(selected_command_str)
                    .show_ui(ui, |ui| {
                        for (i, command) in commands.iter().enumerate() {
                            ui.selectable_value(&mut ui_state.selected_command, Some(i), *command);
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
                            solution.add_rule(Rule::new(*sensor, *command));
                            ui_state.selected_sensor = None;
                            ui_state.selected_command = None;
                        }
                    },
                );
            });

            ui.separator();

            ui.label("Rules:");
            ui.add_space(8.0);

            let mut remove_idx = None;
            for (idx, rule) in solution.rules().iter().enumerate() {
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
                solution.remove_rule(idx);
            }

            if level_complete {
                ui.separator();
                ui.label("Level Complete!");
                if ui.button("Next Level").clicked() {
                    next_level.write(NextLevel);
                }
            }
        });
}

pub struct RuleEditorPlugin;

impl Plugin for RuleEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_systems(EguiPrimaryContextPass, rule_editor_ui);
    }
}
