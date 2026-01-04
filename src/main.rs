use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

impl Dir {
    pub fn rotate_ccw(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }

    pub fn rotate_cw(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
        }
    }

    pub fn to_radians(self) -> f32 {
        match self {
            Dir::East => 0.,
            Dir::North => PI / 2.,
            Dir::West => PI,
            Dir::South => -PI / 2.,
        }
    }
}

impl From<Dir> for Vec2 {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::East => Vec2::new(1., 0.),
            Dir::North => Vec2::new(0., 1.),
            Dir::West => Vec2::new(-1., 0.),
            Dir::South => Vec2::new(0., -1.),
        }
    }
}

#[derive(Clone, Copy)]
enum VacMovementState {
    // base pos, heading
    Moving(Vec2, Dir),

    // starting heading, ending heading
    Rotating(Dir, Dir),
}

#[derive(Clone, Copy)]
enum Dir {
    East,
    North,
    West,
    South,
}
#[derive(Component)]
struct Vac {
    state: VacMovementState,
}

impl Vac {
    fn new(start: Vec2) -> Self {
        Self {
            state: VacMovementState::Moving(start, Dir::East),
        }
    }
}

fn main() {
    let map = Map::parse(MAP_STR);

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
        .insert_resource(map)
        .add_systems(Startup, setup)
        .add_systems(Update, move_vac)
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
    let map_basept = Vec2::new(
        -((map.width / 2) as f32) * GRID_SIZE,
        -((map.height / 2) as f32) * GRID_SIZE,
    );
    for (x, y) in &map.walls {
        let offset = Vec2::new(*x as f32 * GRID_SIZE, *y as f32 * GRID_SIZE);
        let pos = map_basept + offset;

        let wall = meshes.add(Rectangle::new(GRID_SIZE, GRID_SIZE));
        commands.spawn((
            Mesh2d(wall),
            MeshMaterial2d(materials.add(GRAY)),
            Transform::from_translation(pos.extend(0.1)),
        ));
    }

    let (start_x, start_y) = &map.start;
    let offset = Vec2::new(*start_x as f32 * GRID_SIZE, *start_y as f32 * GRID_SIZE);
    let vac_start = map_basept + offset;

    // spawn a circle with a triangle to show heading
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(0.4 * GRID_SIZE))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0., 0., 0.1),
        Vac::new(vac_start),
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
    mut query: Query<(&mut Transform, &mut Vac, &mut MovementTimer)>,
) {
    let (mut transform, mut vac, mut timer) = query.single_mut().unwrap();
    timer.tick(time.delta());

    // if the timer finished since the last update,
    // make sure we're at the destination location, then
    // choose a new direction
    if timer.is_finished() {
        match vac.state {
            VacMovementState::Moving(base_pos, heading) => {
                let heading_vec: Vec2 = heading.into();
                let dest: Vec2 = base_pos + heading_vec * GRID_SIZE;

                // finish moving to the destination point
                transform.translation = dest.extend(0.1);

                // if we aren't facing a wall, move forward.
                // otherwise, rotate
                let pos = transform.translation.xy();
                let next_grid_pt = pos + Vec2::from(heading) * GRID_SIZE;

                let map_basept = Vec2::new(
                    -((map.width / 2) as f32) * GRID_SIZE,
                    -((map.height / 2) as f32) * GRID_SIZE,
                );

                let facing_wall = map.walls.iter().any(|(x, y)| {
                    let offset = Vec2::new(*x as f32 * GRID_SIZE, *y as f32 * GRID_SIZE);
                    let wall_pos = map_basept + offset;

                    wall_pos == next_grid_pt
                });

                if facing_wall {
                    vac.state = if rand::random_bool(0.5) {
                        VacMovementState::Rotating(heading, heading.rotate_cw())
                    } else {
                        VacMovementState::Rotating(heading, heading.rotate_ccw())
                    };
                } else {
                    vac.state = VacMovementState::Moving(pos, heading);
                }
            }
            VacMovementState::Rotating(_, end) => {
                // if we aren't facing a wall, move forward.
                // otherwise, keep rotating
                let pos = transform.translation.xy();
                let next_grid_pt = pos + Vec2::from(end) * GRID_SIZE;

                let map_basept = Vec2::new(
                    -((map.width / 2) as f32) * GRID_SIZE,
                    -((map.height / 2) as f32) * GRID_SIZE,
                );

                let facing_wall = map.walls.iter().any(|(x, y)| {
                    let offset = Vec2::new(*x as f32 * GRID_SIZE, *y as f32 * GRID_SIZE);
                    let wall_pos = map_basept + offset;

                    wall_pos == next_grid_pt
                });

                if facing_wall {
                    vac.state = if rand::random_bool(0.5) {
                        VacMovementState::Rotating(end, end.rotate_cw())
                    } else {
                        VacMovementState::Rotating(end, end.rotate_ccw())
                    };
                } else {
                    vac.state = VacMovementState::Moving(pos, end);
                }
            }
        }
    } else {
        let elapsed = timer.elapsed().as_millis() as f32 / STEP_TIME_MS as f32;
        match vac.state {
            VacMovementState::Moving(base_pos, heading) => {
                let heading_vec: Vec2 = heading.into();
                let dest: Vec2 = base_pos + heading_vec * GRID_SIZE;

                let pos = Vec2::lerp(base_pos, dest, elapsed);
                transform.translation = pos.extend(0.1);
            }
            VacMovementState::Rotating(start, end) => {
                let start = Quat::from_rotation_z(start.to_radians());
                let end = Quat::from_rotation_z(end.to_radians());
                transform.rotation = Quat::slerp(start, end, elapsed);
            }
        }
    }
}

const MAP_STR: &str = r"#######
#...###
#.#.###
#S....#
#.#.#.#
#.#...#
#.###.#
#.....#
#######
";

#[derive(Debug, Resource)]
struct Map {
    walls: Vec<(usize, usize)>,

    // (x,y) pairs with y increasing upwards
    start: (usize, usize),

    width: usize,
    height: usize,
}

impl Map {
    pub fn parse(str: &str) -> Self {
        let mut walls = Vec::new();
        let mut start = (0, 0);

        let width = str.lines().next().unwrap().len();
        let height = str.lines().count();

        for (row_idx, row) in str.lines().enumerate() {
            for (col_idx, char) in row.chars().enumerate() {
                match char {
                    '#' => {
                        walls.push((col_idx, height - row_idx - 1));
                    }
                    'S' => {
                        start = (col_idx, height - row_idx - 1);
                    }
                    _ => {}
                }
            }
        }

        Self {
            walls,
            start,

            width,
            height,
        }
    }
}
