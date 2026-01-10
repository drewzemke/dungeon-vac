use bevy::prelude::*;

use crate::{
    core::{
        dir::Dir,
        state::{Effect, State as CoreState},
    },
    game::{
        constants::GRID_SIZE,
        map::{Map, MapSetup},
        simulation::Simulation,
    },
    ui::rule_editor::Rules,
};

const STEP_TIME_MS: u64 = 500;

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

#[derive(Component, Deref, DerefMut)]
struct VacMovementTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct State(CoreState);

impl VacMovementTimer {
    fn new() -> Self {
        let timer = Timer::new(
            std::time::Duration::from_millis(STEP_TIME_MS),
            TimerMode::Repeating,
        );
        Self(timer)
    }
}

fn setup_vac(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rules: Res<Rules>,
    map: Query<&Map>,
) {
    let map = map.single().unwrap();
    let mut state = CoreState::new(map.start(), Dir::East);

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
        VacMovementTimer::new(),
        State(state),
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

fn move_vac(
    mut query: Query<(&mut Transform, &mut Vac, &mut VacMovementTimer, &mut State)>,
    map: Query<&Map>,
    rules: ResMut<Rules>,
    time: Res<Time>,
    sim: Res<Simulation>,
) {
    if !sim.is_running() {
        return;
    }

    let (mut transform, mut vac, mut timer, mut state) = query.single_mut().unwrap();
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

pub struct VacPlugin;

impl Plugin for VacPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_vac.after(MapSetup))
            .add_systems(Update, move_vac);
    }
}
