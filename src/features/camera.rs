use crate::features::input::{CameraPanAction, CameraZoomAction};
use crate::features::movement::{Acceleration, Friction, Velocity};
use crate::features::position::{PreviousWorldPosition, WorldPosition};
use bevy::app::RunFixedMainLoopSystem::BeforeFixedMainLoop;
use bevy::app::{App, Plugin, RunFixedMainLoop, Startup};
use bevy::ecs::prelude::*;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_atmosphere::plugin::{AtmosphereCamera, AtmospherePlugin};
use leafwing_input_manager::prelude::*;
use std::ops::{Add, Sub};

pub struct CameraPlugin;

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(pub Vec2);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_systems(Startup, setup)
            .add_systems(
                RunFixedMainLoop,
                handle_pan_input.in_set(BeforeFixedMainLoop),
            )
            .add_systems(Update, handle_zoom_input);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pan_input_map = InputMap::new([
        (CameraPanAction::Left, KeyA),
        (CameraPanAction::Right, KeyD),
        (CameraPanAction::Up, KeyW),
        (CameraPanAction::Down, KeyS),
    ]);

    let zoom_input_map = InputMap::default()
        .with(CameraZoomAction::In, MouseScrollDirection::UP)
        .with(CameraZoomAction::In, KeyCode::ArrowUp)
        .with(CameraZoomAction::Out, MouseScrollDirection::DOWN);

    // camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 48.0,
            },
            near: -100.0,
            far: 200.0,
            ..OrthographicProjection::default_3d()
        }),
        //ScreenSpaceAmbientOcclusion::default(),
        Msaa::Off,
        AtmosphereCamera::default(),
        // Skybox {
        //     image: sky_image_handle.clone(),
        //     brightness: 6000.0,
        //     rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2 / 1.5),
        // },
        Transform::from_xyz(0.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        InputManagerBundle::with_map(pan_input_map),
        InputManagerBundle::with_map(zoom_input_map),
        RayCastPickable,
        Velocity::default(),
        Acceleration::default(),
        WorldPosition::default(),
        PreviousWorldPosition::default(),
        AccumulatedInput::default(),
        Friction(0.5),
    ));
}

fn handle_pan_input(
    mut query: Query<
        (
            &ActionState<CameraPanAction>,
            &mut AccumulatedInput,
            &mut Acceleration,
        ),
        With<Camera3d>,
    >,
) {
    let accel_speed = 673.0;
    for (action_state, mut accumulated_input, mut acceleration) in &mut query {
        if action_state.pressed(&CameraPanAction::Left) {
            accumulated_input.x = -1.0;
        }

        if action_state.pressed(&CameraPanAction::Right) {
            accumulated_input.x = 1.0;
        }

        if action_state.pressed(&CameraPanAction::Up) {
            accumulated_input.y = -1.0;
        }

        if action_state.pressed(&CameraPanAction::Down) {
            accumulated_input.y = 1.0;
        }

        acceleration.0 = accumulated_input.normalize_or_zero() * accel_speed;
    }
}

fn handle_zoom_input(
    mut query: Query<(&ActionState<CameraZoomAction>, &mut Projection)>,
    time: Res<Time>,
) {
    let zoom_amount = 0.3;
    for (action_state, mut camera_projection) in &mut query {
        if let Projection::Orthographic(ref mut ortho_projection) = *camera_projection.into_inner()
        {
            if action_state.pressed(&CameraZoomAction::In) {
                ortho_projection.scale = ortho_projection.scale.add(zoom_amount).min(3.0);
            }

            if action_state.pressed(&CameraZoomAction::Out) {
                ortho_projection.scale = ortho_projection.scale.sub(zoom_amount).max(0.4);
            }
        }
    }
}
