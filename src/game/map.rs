use bevy::prelude::*;

use crate::{core::map::Map as CoreMap, game::constants::GRID_SIZE};

#[derive(Debug, Component)]
pub struct Map {
    map: CoreMap,

    base_pt: Vec2,
}

impl std::ops::Deref for Map {
    type Target = CoreMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl std::ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl Map {
    pub fn new(map: CoreMap) -> Self {
        let (width, height) = map.dimensions();
        let base_pt = Vec2::new(
            -((width / 2) as f32) * GRID_SIZE,
            -((height / 2) as f32) * GRID_SIZE,
        );

        Self { map, base_pt }
    }

    pub fn to_game_world(&self, pt: impl ToGameWorld) -> Vec3 {
        let pt = pt.to_game_pt();
        let offset = Vec2::new(pt.x * GRID_SIZE, pt.y * GRID_SIZE);
        (self.base_pt + offset).extend(0.0)
    }
}

/// Helper trait so that `Map::to_game_world` can be passed either Vec2 or IVec2
pub trait ToGameWorld {
    fn to_game_pt(&self) -> Vec2;
}

impl ToGameWorld for IVec2 {
    fn to_game_pt(&self) -> Vec2 {
        self.as_vec2()
    }
}

impl ToGameWorld for Vec2 {
    fn to_game_pt(&self) -> Vec2 {
        *self
    }
}

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

pub const WALL_COLOR: Color = Color::hsl(0., 0.0, 0.3);

pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let map = Map::new(CoreMap::parse(MAP_STR).unwrap());

    let wall_positions = map
        .walls()
        .iter()
        .map(|wall| map.to_game_world(*wall))
        .collect::<Vec<_>>();

    // spawn map with wall tiles as children
    commands
        .spawn((map, Transform::default(), Visibility::default()))
        .with_children(|parent| {
            for wall_pos in wall_positions {
                let wall = meshes.add(Rectangle::new(GRID_SIZE, GRID_SIZE));

                parent.spawn((
                    Mesh2d(wall),
                    MeshMaterial2d(materials.add(WALL_COLOR)),
                    Transform::from_translation(wall_pos),
                ));
            }
        });
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapSetup;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map.in_set(MapSetup));
    }
}
