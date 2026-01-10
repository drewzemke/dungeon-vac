use bevy::prelude::*;

use crate::game::constants::GRID_SIZE;

const NUM_GRID_LINES: i64 = 10;
const RED: Color = Color::hsl(0., 0.95, 0.7);
const BLUE: Color = Color::hsl(200., 0.95, 0.7);

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid);
    }
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let horizontal_lines = (-NUM_GRID_LINES..NUM_GRID_LINES)
        .map(|i| {
            meshes.add(Polyline2d::new([
                Vec2::new(-(NUM_GRID_LINES as f32) * GRID_SIZE, i as f32 * GRID_SIZE),
                Vec2::new(NUM_GRID_LINES as f32 * GRID_SIZE, i as f32 * GRID_SIZE),
            ]))
        })
        .collect::<Vec<_>>();

    let vertical_lines = (-NUM_GRID_LINES..NUM_GRID_LINES)
        .map(|i| {
            meshes.add(Polyline2d::new([
                Vec2::new(i as f32 * GRID_SIZE, -(NUM_GRID_LINES as f32) * GRID_SIZE),
                Vec2::new(i as f32 * GRID_SIZE, NUM_GRID_LINES as f32 * GRID_SIZE),
            ]))
        })
        .collect::<Vec<_>>();

    for mesh in horizontal_lines {
        commands.spawn((
            Transform::from_xyz(0., 0., -0.1),
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(RED)),
        ));
    }

    for mesh in vertical_lines {
        commands.spawn((
            Transform::from_xyz(0., 0., -0.1),
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(BLUE)),
        ));
    }
}
