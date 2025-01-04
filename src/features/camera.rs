use crate::features::input::CameraPanAction;
use crate::features::movement::{
    Acceleration, Friction, PreviousWorldPosition, Velocity, WorldPosition,
};
use bevy::app::RunFixedMainLoopSystem::BeforeFixedMainLoop;
use bevy::app::{App, Plugin, RunFixedMainLoop, Startup};
use bevy::ecs::prelude::*;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::{
    Camera3d, Commands, Deref, DerefMut, OrthographicProjection, Projection, Transform,
};
use bevy::render::camera::ScalingMode;
use leafwing_input_manager::prelude::*;

pub struct CameraPlugin;

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(pub Vec2);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(RunFixedMainLoop, handle_input.in_set(BeforeFixedMainLoop));
    }
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::new([
        (CameraPanAction::Left, KeyA),
        (CameraPanAction::Right, KeyD),
        (CameraPanAction::Up, KeyW),
        (CameraPanAction::Down, KeyS),
    ]);

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
        Transform::from_xyz(0.0, 30.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        InputManagerBundle::with_map(input_map),
        Velocity::default(),
        Acceleration::default(),
        WorldPosition::default(),
        PreviousWorldPosition::default(),
        AccumulatedInput::default(),
        Friction(0.5),
    ));
}

fn handle_input(
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
