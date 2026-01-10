use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use dungeon_vac::{
    core::{
        command::Command,
        dir::Dir,
        map::Map as GameMap,
        rule::Rule,
        sensor::Sensor,
        state::{Effect, State as GameState},
    },
    game::{
        constants::GRID_SIZE,
        map::{Map, MapPlugin, MapSetup},
    },
    ui::{
        grid::GridPlugin,
        rule_editor::{RuleEditor, Rules, rule_editor_ui},
    },
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
    Rule::new(Sensor::SpaceRight, Command::TurnRight),
    Rule::new(Sensor::HitWall, Command::TurnLeft),
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

#[derive(Resource, Deref, DerefMut)]
struct State(GameState);

fn main() {
    let map = GameMap::parse(MAP_STR).unwrap();
    let state = State(GameState::new(map.start(), Dir::East));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dungeon Vac".to_string(),
                resolution: (900, 600).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(MapPlugin)
        .add_plugins(GridPlugin)
        .insert_resource(state)
        .insert_resource(Rules(Vec::from(RULES)))
        .init_resource::<RuleEditor>()
        .add_systems(Startup, setup.after(MapSetup))
        .add_systems(Update, move_vac)
        .add_systems(EguiPrimaryContextPass, rule_editor_ui)
        .run();
}

const STEP_TIME_MS: u64 = 500;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<State>,
    rules: Res<Rules>,
    map: Query<&Map>,
) {
    commands.spawn(Camera2d);

    let map = map.single().unwrap();

    // compute starting map location
    let initial_pos = map.to_game_world(state.vac_pos());

    // execute initial tick
    let effect = state.tick(map, &rules);
    let vac = Vac::new(effect);

    // spawn a circle with a triangle to show heading
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(0.4 * GRID_SIZE))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_translation(initial_pos),
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
    rules: ResMut<Rules>,
    mut state: ResMut<State>,
    mut query: Query<(&mut Transform, &mut Vac, &mut MovementTimer)>,
    map: Query<&Map>,
) {
    let (mut transform, mut vac, mut timer) = query.single_mut().unwrap();
    let map = map.single().unwrap();
    timer.tick(time.delta());

    // if the timer finished since the last update,
    // make sure we're at the destination location, then
    // choose a new direction
    if timer.is_finished() {
        // finish moving to the destination point
        transform.translation = map.to_game_world(state.vac_pos());

        // update state and store in movement state
        let effect = state.tick(map, &rules);
        vac.effect = effect;
    } else {
        let elapsed = timer.elapsed().as_millis() as f32 / STEP_TIME_MS as f32;

        match vac.effect {
            Effect::Moved { from, to } => {
                let pos = Vec2::lerp(from.as_vec2(), to.as_vec2(), elapsed);
                transform.translation = map.to_game_world(pos);
            }
            Effect::Rotated { from, to } => {
                let from = Quat::from_rotation_z(from.to_radians());
                let to = Quat::from_rotation_z(to.to_radians());
                transform.rotation = Quat::slerp(from, to, elapsed);
            }
            Effect::BumpedWall => {
                let bump_direction = Vec2::from(state.vac_dir());

                let bump_offset = if elapsed < 0.3 {
                    // phase 1: move forward at usual speed
                    let progress = elapsed / 0.3;
                    bump_direction * 0.2 * progress
                } else if elapsed < 0.7 {
                    // phase 2: bounce back
                    let progress = (elapsed - 0.3) / 0.4;
                    let forward = 0.2;
                    let back = -0.15;
                    bump_direction * (forward + (back - forward) * progress)
                } else {
                    // phase 3: small rebound forward to settle
                    let progress = (elapsed - 0.7) / 0.3;
                    let back = -0.15;
                    bump_direction * (back + (0.0 - back) * progress)
                };

                transform.translation = map.to_game_world(state.vac_pos().as_vec2() + bump_offset);
            }
        }
    }
}
