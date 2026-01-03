use std::time::Duration;

use bevy::prelude::*;

// const MOVE_SPEED: f32 = 300.0;

#[derive(Component)]
struct Vac {
    base_pos: Vec2,
    heading: Vec2,
}

impl Vac {
    fn new() -> Self {
        Self {
            base_pos: Vec2::ZERO,
            heading: Vec2::new(1., 0.),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dungeon Vac".to_string(),
                resolution: (800, 600).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_vac)
        .run();
}

const NUM_GRID_LINES: i64 = 10;
const GRID_SIZE: f32 = 50.;

const RED: Color = Color::hsl(0., 0.95, 0.7);
const BLUE: Color = Color::hsl(200., 0.95, 0.7);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // spawn a circle
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(0.4 * GRID_SIZE))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Vac::new(),
        MovementTimer::new(),
    ));

    // draw a grid
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
        commands.spawn((Mesh2d(mesh), MeshMaterial2d(materials.add(RED))));
    }

    for mesh in vertical_lines {
        commands.spawn((Mesh2d(mesh), MeshMaterial2d(materials.add(BLUE))));
    }
}

#[derive(Component, Deref, DerefMut)]
struct MovementTimer(Timer);

impl MovementTimer {
    fn new() -> Self {
        let timer = Timer::new(Duration::from_millis(1000), TimerMode::Repeating);
        Self(timer)
    }
}

fn move_vac(time: Res<Time>, mut query: Query<(&mut Transform, &mut Vac, &mut MovementTimer)>) {
    let (mut transform, mut vac, mut timer) = query.single_mut().unwrap();
    timer.tick(time.delta());

    // if the timer finished since the last update,
    // make sure we're at the destination location, then
    // choose a new direction
    let dest = vac.base_pos + vac.heading * GRID_SIZE;
    if timer.is_finished() {
        transform.translation = dest.extend(0.0);

        vac.base_pos = dest;
        vac.heading = Vec2::new(0., -1.);
    } else {
        let elapsed = timer.elapsed_secs_f64() as f32;
        let pos = Vec2::lerp(vac.base_pos, dest, elapsed);
        transform.translation = pos.extend(0.0);
    }
}
