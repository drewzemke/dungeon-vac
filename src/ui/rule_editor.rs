use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{
    core::{command::Command as GameCommand, rule::Rule, sensor::Sensor},
    game::simulation::Simulation,
};

#[derive(Default, Resource)]
pub struct RuleEditor {
    pub selected_sensor: usize,
    pub selected_command: usize,
}

// FIXME: this isn't the right place for this
#[derive(Default, Resource, Deref, DerefMut)]
pub struct Rules(pub Vec<Rule>);

pub fn rule_editor_ui(
    mut contexts: EguiContexts,
    mut editor: ResMut<RuleEditor>,
    mut rules: ResMut<Rules>,
    mut sim: ResMut<Simulation>,
) {
    let sensors = [Sensor::HitWall, Sensor::SpaceLeft, Sensor::SpaceRight];
    let commands = [GameCommand::TurnRight, GameCommand::TurnLeft];

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let running = sim.is_running();

    egui::SidePanel::left("rule_editor")
        .resizable(false)
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_enabled_ui(!running, |ui| {
                    if ui.button("Start").clicked() {
                        sim.start();
                    }
                });

                ui.add_enabled_ui(running, |ui| {
                    if ui.button("Stop").clicked() {
                        sim.stop();
                    }
                });
            });

            ui.separator();

            ui.label("Create Rule:");
            ui.add_space(8.0);

            ui.add_enabled_ui(!running, |ui| {
                egui::ComboBox::from_label("Sensor")
                    .selected_text(sensors[editor.selected_sensor])
                    .show_ui(ui, |ui| {
                        for (i, sensor) in sensors.iter().enumerate() {
                            ui.selectable_value(&mut editor.selected_sensor, i, *sensor);
                        }
                    });

                egui::ComboBox::from_label("Command")
                    .selected_text(commands[editor.selected_command])
                    .show_ui(ui, |ui| {
                        for (i, command) in commands.iter().enumerate() {
                            ui.selectable_value(&mut editor.selected_command, i, *command);
                        }
                    });

                ui.add_space(8.0);
                if ui.button("Add Rule").clicked() {
                    let sensor = sensors[editor.selected_sensor];
                    let command = commands[editor.selected_command];
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
                    ui.add_enabled_ui(!running, |ui| {
                        if ui.button("X").clicked() {
                            remove_idx = Some(idx);
                        }
                    });
                });
            }

            if let Some(idx) = remove_idx {
                rules.remove(idx);
            }
        });
}
