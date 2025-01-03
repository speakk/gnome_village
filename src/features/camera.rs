use bevy::app::{App, Plugin, Startup};
use bevy::math::Vec3;
use bevy::prelude::{Camera3d, Commands, OrthographicProjection, Projection, Transform};
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 48.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(20.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}