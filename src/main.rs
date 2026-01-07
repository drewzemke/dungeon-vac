use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use dungeon_vac::{
    game::{
        action::Action,
        dir::Dir,
        event::Event as GameEvent,
        level::Level,
        rule::Rule,
        state::{Effect, State as GameState},
    },
    ui::rule_editor::{RuleEditor, rule_editor_ui},
};

#[derive(Component)]
struct Vac {
    effect: Effect,
}

impl Vac {
    fn new(initial_effect: Effect) -> Self {
        Self {
            effect: initial_effect,
        }
    }
}

const RULES: [Rule; 2] = [
    Rule::new(GameEvent::SpaceRight, Action::TurnRight),
    Rule::new(GameEvent::HitWall, Action::TurnLeft),
];

const MAP_STR: &str = r"#######
#S..###
#.#.###
#.#...#
#.#.#.#
#.#...#
#.###.#
#.....#
#######
";

#[derive(Resource, Deref)]
struct Map(Level);

#[derive(Resource, Deref, DerefMut)]
struct State(GameState);

fn main() {
    let map = Map(Level::parse(MAP_STR).unwrap());
    let state = State(GameState::new(map.start(), Dir::East));

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
        .add_plugins(EguiPlugin::default())
        .insert_resource(map)
        .insert_resource(state)
        .init_resource::<RuleEditor>()
        .add_systems(Startup, setup)
        .add_systems(Update, move_vac)
        .add_systems(EguiPrimaryContextPass, rule_editor_ui)
        .run();
}

const NUM_GRID_LINES: i64 = 10;
const GRID_SIZE: f32 = 50.;
const STEP_TIME_MS: u64 = 500;

const RED: Color = Color::hsl(0., 0.95, 0.7);
const BLUE: Color = Color::hsl(200., 0.95, 0.7);
const GRAY: Color = Color::hsl(0., 0.0, 0.3);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<State>,
    map: Res<Map>,
) {
    commands.spawn(Camera2d);

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

    // draw the map walls
    let (width, height) = map.dimensions();
    let map_basept = Vec2::new(
        -((width / 2) as f32) * GRID_SIZE,
        -((height / 2) as f32) * GRID_SIZE,
    );
    for wall_pos in map.walls() {
        let x = wall_pos.x as f32;
        let y = wall_pos.y as f32;
        let offset = Vec2::new(x * GRID_SIZE, y * GRID_SIZE);
        let pos = map_basept + offset;

        let wall = meshes.add(Rectangle::new(GRID_SIZE, GRID_SIZE));
        commands.spawn((
            Mesh2d(wall),
            MeshMaterial2d(materials.add(GRAY)),
            Transform::from_translation(pos.extend(0.1)),
        ));
    }

    // compute starting map location
    let initial_pos = map_basept + state.vac_pos().as_vec2() * GRID_SIZE;

    // execute initial tick
    let effect = state.tick(&map, &RULES);
    let vac = Vac::new(effect);

    // spawn a circle with a triangle to show heading
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(0.4 * GRID_SIZE))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_translation(initial_pos.extend(0.1)),
        vac,
        MovementTimer::new(),
        // triangle
        children![(
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::new(0., 0.2 * GRID_SIZE),
                Vec2::new(0., -0.2 * GRID_SIZE),
                Vec2::new(0.2 * GRID_SIZE, 0.),
            ))),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_xyz(0.2 * GRID_SIZE, 0., 0.),
        )],
    ));
}

#[derive(Component, Deref, DerefMut)]
struct MovementTimer(Timer);

impl MovementTimer {
    fn new() -> Self {
        let timer = Timer::new(Duration::from_millis(STEP_TIME_MS), TimerMode::Repeating);
        Self(timer)
    }
}

fn move_vac(
    time: Res<Time>,
    map: Res<Map>,
    mut state: ResMut<State>,
    mut query: Query<(&mut Transform, &mut Vac, &mut MovementTimer)>,
) {
    let (mut transform, mut vac, mut timer) = query.single_mut().unwrap();
    timer.tick(time.delta());

    // if the timer finished since the last update,
    // make sure we're at the destination location, then
    // choose a new direction
    if timer.is_finished() {
        // FIXME: how to avoid doing this every time?
        // make the map wrapper more rich, so that it include the basepoint?
        // or have it do the translation for us even??
        let (width, height) = map.dimensions();
        let map_basept = Vec2::new(
            -((width / 2) as f32) * GRID_SIZE,
            -((height / 2) as f32) * GRID_SIZE,
        );

        let prev_pos = map_basept + state.vac_pos().as_vec2() * GRID_SIZE;

        // finish moving to the destination point
        transform.translation = prev_pos.extend(0.1);

        // update state and store in movement state
        let effect = state.tick(&map, &RULES);
        vac.effect = effect;
    } else {
        let elapsed = timer.elapsed().as_millis() as f32 / STEP_TIME_MS as f32;

        let (width, height) = map.dimensions();
        let map_basept = Vec2::new(
            -((width / 2) as f32) * GRID_SIZE,
            -((height / 2) as f32) * GRID_SIZE,
        );

        match vac.effect {
            Effect::Moved { from, to } => {
                let from = map_basept + from.as_vec2() * GRID_SIZE;
                let to = map_basept + to.as_vec2() * GRID_SIZE;
                let pos = Vec2::lerp(from, to, elapsed);
                transform.translation = pos.extend(0.1);
            }
            Effect::Rotated { from, to } => {
                let from = Quat::from_rotation_z(from.to_radians());
                let to = Quat::from_rotation_z(to.to_radians());
                transform.rotation = Quat::slerp(from, to, elapsed);
            }
            Effect::BumpedWall => {
                let (width, height) = map.dimensions();
                let map_basept = Vec2::new(
                    -((width / 2) as f32) * GRID_SIZE,
                    -((height / 2) as f32) * GRID_SIZE,
                );

                let base_pos = map_basept + state.vac_pos().as_vec2() * GRID_SIZE;
                let bump_direction = Vec2::from(state.vac_dir());

                let bump_offset = if elapsed < 0.3 {
                    // phase 1: move forward at usual speed
                    let progress = elapsed / 0.3;
                    bump_direction * 0.2 * GRID_SIZE * progress
                } else if elapsed < 0.7 {
                    // phase 2: bounce back
                    let progress = (elapsed - 0.3) / 0.4;
                    let forward = 0.2 * GRID_SIZE;
                    let back = -0.15 * GRID_SIZE;
                    bump_direction * (forward + (back - forward) * progress)
                } else {
                    // phase 3: small rebound forward to settle
                    let progress = (elapsed - 0.7) / 0.3;
                    let back = -0.15 * GRID_SIZE;
                    bump_direction * (back + (0.0 - back) * progress)
                };

                transform.translation = (base_pos + bump_offset).extend(0.1);
            }
        }
    }
}
