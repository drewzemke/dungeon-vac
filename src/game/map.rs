use bevy::prelude::*;

use crate::{
    core::map::Map as CoreMap,
    game::{constants::GRID_SIZE, level::LevelProgression},
    messages::{LoadLevel, TrashCollected},
};

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

pub const WALL_COLOR: Color = Color::hsl(0., 0.0, 0.3);
pub const TRASH_COLOR: Color = Color::hsl(90., 0.1, 0.7);
pub const EXIT_COLOR: Color = Color::hsl(55., 0.9, 0.6);

// TODO: move this to a separate module?
/// marker component to make it easier to despawn trash
#[derive(Component)]
pub struct Trash(IVec2);

/// called when a new level has been selected
pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    levels: Res<LevelProgression>,
    current_map: Query<Entity, With<Map>>,
) {
    // despawn the current map if there is one
    if let Ok(current) = current_map.single() {
        commands.entity(current).despawn();
    }

    let level = levels.current();
    let map = level.map();
    let map = Map::new(map.clone());

    let wall_positions = map
        .walls()
        .iter()
        .map(|wall| map.to_game_world(*wall))
        .collect::<Vec<_>>();

    let trash_positions = map
        .trash()
        .iter()
        .map(|trash| (*trash, map.to_game_world(*trash)))
        .collect::<Vec<_>>();

    let exit_pos = map.to_game_world(map.exit());

    // spawn map with tiles as children
    commands
        .spawn((map, Transform::default(), Visibility::default()))
        .with_children(|parent| {
            // wall tiles
            for wall_pos in wall_positions {
                let wall = meshes.add(Rectangle::new(GRID_SIZE, GRID_SIZE));

                parent.spawn((
                    Mesh2d(wall),
                    MeshMaterial2d(materials.add(WALL_COLOR)),
                    Transform::from_translation(wall_pos),
                ));
            }

            // trash tiles
            for (trash_pos2, trash_pos3) in trash_positions {
                let trash = meshes.add(Rectangle::new(GRID_SIZE * 0.8, GRID_SIZE * 0.8));

                parent.spawn((
                    Trash(trash_pos2),
                    Mesh2d(trash),
                    MeshMaterial2d(materials.add(TRASH_COLOR)),
                    Transform::from_translation(trash_pos3),
                ));
            }

            // exit tile
            let exit = meshes.add(Circle::new(GRID_SIZE * 0.45));

            parent.spawn((
                Mesh2d(exit),
                MeshMaterial2d(materials.add(EXIT_COLOR)),
                Transform::from_translation(exit_pos),
            ));
        });
}

fn despawn_trash(
    mut commands: Commands,
    trash: Query<(Entity, &Trash)>,
    mut reader: MessageReader<TrashCollected>,
) {
    for TrashCollected(pt) in reader.read() {
        // find the corresponding entity and despawn it
        let trash = trash.iter().find(|(_, Trash(pt2))| *pt == *pt2);
        if let Some((entity, _)) = trash {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapSetup;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_map.in_set(MapSetup).run_if(on_message::<LoadLevel>),
                despawn_trash.run_if(on_message::<TrashCollected>),
            ),
        );
    }
}
