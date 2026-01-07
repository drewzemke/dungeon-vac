use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Default, Resource)]
pub struct RuleEditor {
    pub selected_sensor: usize,
    pub selected_command: usize,
}

pub fn rule_editor_ui(mut contexts: EguiContexts, mut editor: ResMut<RuleEditor>) {
    let sensors = [
        "WHEN hit wall",
        "WHEN space left",
        "WHEN space right",
        "WHEN start",
    ];
    let commands = ["THEN turn left", "THEN turn right", "THEN clean on"];

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
                println!(
                    "Add rule: {} -> {}",
                    sensors[editor.selected_sensor], commands[editor.selected_command]
                );
            }
        });
}
