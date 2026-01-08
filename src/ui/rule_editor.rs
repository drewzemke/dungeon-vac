use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::game::{command::Command as GameCommand, rule::Rule, sensor::Sensor};

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
) {
    let sensors = [Sensor::HitWall, Sensor::SpaceLeft, Sensor::SpaceRight];
    let commands = [GameCommand::TurnRight, GameCommand::TurnLeft];

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::SidePanel::left("rule_editor")
        .resizable(false)
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Rules");
            ui.separator();

            ui.label("Create Rule:");
            ui.add_space(8.0);

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

            ui.separator();

            ui.label("Rules:");
            ui.add_space(8.0);

            let mut remove_idx = None;
            for (idx, rule) in rules.0.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{} {}", rule.sensor(), rule.command()));
                    if ui.button("X").clicked() {
                        remove_idx = Some(idx);
                    }
                });
            }

            if let Some(idx) = remove_idx {
                rules.remove(idx);
            }
        });
}
