use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (zoom_camera, pan_camera));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default()));
}

fn pan_camera(
    mut camera: Query<(&mut Transform, &Projection), With<Camera2d>>,
    motion: Res<AccumulatedMouseMotion>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.pressed(MouseButton::Right) {
        return;
    }

    let (mut transform, proj) = camera.single_mut().unwrap();

    let scale = if let Projection::Orthographic(proj) = proj {
        proj.scale
    } else {
        1.0
    };

    let delta = motion.delta * scale;

    transform.translation += Vec3::new(-delta.x, delta.y, 0.);
}

const MAX_ZOOM: f32 = 2.;
const MIN_ZOOM: f32 = 0.5;
const SCROLL_FACTOR: f32 = 0.02;

fn zoom_camera(
    mut camera_proj: Query<&mut Projection, With<Camera2d>>,
    scroll: Res<AccumulatedMouseScroll>,
) {
    let mut proj = camera_proj.single_mut().unwrap();

    if let Projection::Orthographic(proj) = &mut *proj {
        proj.scale = (proj.scale + SCROLL_FACTOR * scroll.delta.y).clamp(MIN_ZOOM, MAX_ZOOM);
    }
}
