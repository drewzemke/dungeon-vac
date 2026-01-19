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
        solution::Solution,
        state::State as GameState,
    },
    messages::{LevelComplete, LoadLevel, StartSimulation, TrashCollected},
};

const STEP_TIME_MS: u64 = 500;

#[derive(Component)]
struct Vac {
    effect: Option<Effect>,
}

impl Vac {
    fn new() -> Self {
        Self { effect: None }
    }

    fn with_effect(effect: Effect) -> Self {
        Self {
            effect: Some(effect),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct VacMovementTimer(Timer);

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
    map: Query<&Map>,
    vac: Query<Entity, With<Vac>>,
) {
    // despawn the current vac if there is one
    if let Ok(vac) = vac.single() {
        commands.entity(vac).despawn();
    }

    let map = map.single().unwrap();
    let initial_pos = map.to_game_world(map.start());

    // spawn a circle with a triangle to show heading
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(0.4 * GRID_SIZE))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_translation(initial_pos),
        Vac::new(),
        VacMovementTimer::new(),
        GameState::new(),
        // triangle
        children![(
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::new(0., 0.2 * GRID_SIZE),
                Vec2::new(0., -0.2 * GRID_SIZE),
                Vec2::new(0.2 * GRID_SIZE, 0.),
            ))),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_xyz(0.2 * GRID_SIZE, 0., 0.1),
        )],
    ));
}

fn on_start_sim(
    solution: Res<Solution>,
    map: Query<&Map>,
    mut query: Query<(&mut Vac, &mut GameState)>,
) {
    let (mut vac, mut game_state) = query.single_mut().unwrap();

    // state gets initialized to `None` in setup, so we should hydrate
    // it with the current map and execute the first tick
    // only if it's currently `None`. otherwise, this event was
    // probably a resume-after-pause, so don't do anything
    if game_state.is_some() {
        return;
    }

    let map = map.single().unwrap();

    let mut state = CoreState::new(map.start(), Dir::East);

    // execute initial tick
    let effect = state.tick(map, solution.rules());
    let new_vac = Vac::with_effect(effect);

    *game_state = GameState::with_state(state);
    *vac = new_vac;
}

fn move_vac(
    mut query: Query<(
        &mut Transform,
        &mut Vac,
        &mut VacMovementTimer,
        &mut GameState,
    )>,
    map: Query<&Map>,
    solution: Res<Solution>,
    time: Res<Time>,
    sim: Res<Simulation>,
    mut level_complete: MessageWriter<LevelComplete>,
    mut trash_collected: MessageWriter<TrashCollected>,
) {
    if !sim.is_running() {
        return;
    }

    let (mut transform, mut vac, mut timer, mut state) = query.single_mut().unwrap();

    let Some(state) = &mut **state else { return };
    let map = map.single().unwrap();

    timer.tick(time.delta());

    // if the timer finished since the last update,
    // make sure we're at the destination location, then
    // choose a new direction
    if timer.is_finished() {
        // finish moving to the destination point
        transform.translation = map.to_game_world(state.vac_pos());

        // update state and store in movement state
        let effect = state.tick(map, solution.rules());
        vac.effect = Some(effect);

        // fire an event if we just collected trash
        if let Effect::Moved {
            from,
            collected_trash: true,
            ..
        } = effect
        {
            trash_collected.write(TrashCollected(from));
        }
    } else {
        let elapsed = timer.elapsed().as_millis() as f32 / STEP_TIME_MS as f32;

        let Some(effect) = vac.effect else {
            return;
        };

        match effect {
            Effect::Moved { from, to, .. } => {
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
            Effect::Exited => {
                level_complete.write(LevelComplete);
            }
        }
    }
}

pub struct VacPlugin;

impl Plugin for VacPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_vac.after(MapSetup).run_if(on_message::<LoadLevel>),
                on_start_sim.run_if(on_message::<StartSimulation>),
                move_vac,
            ),
        );
    }
}
