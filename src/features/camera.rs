use crate::features::input::{CameraPanAction, CameraZoomAction};
use crate::features::movement::{Acceleration, Friction, Velocity};
use crate::features::position::InterpolatePosition;
use crate::features::position::WorldPosition;
use crate::features::states::AppState;
use bevy::app::RunFixedMainLoopSystem::BeforeFixedMainLoop;
use bevy::app::{App, Plugin, RunFixedMainLoop};
use bevy::core_pipeline::prepass::{DeferredPrepass, DepthPrepass, NormalPrepass};
use bevy::ecs::prelude::*;
use bevy::math::{Vec2, Vec3};
use bevy::pbr::ClusterConfig;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, ScalingMode};
use bevy_atmosphere::plugin::{AtmosphereCamera, AtmospherePlugin};
use leafwing_input_manager::prelude::*;
use moonshine_core::save::Save;
use moonshine_object::Object;
use moonshine_view::{BuildView, RegisterView, ViewCommands};
use std::ops::{Add, Sub};

pub struct CameraPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(WorldPosition, Velocity, Acceleration, Friction(|| Friction(2.0)), AccumulatedInput, InterpolatePosition)]
pub struct WorldCamera;

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(pub Vec2);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_viewable::<WorldCamera>()
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                RunFixedMainLoop,
                handle_pan_input
                    .in_set(BeforeFixedMainLoop)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, handle_zoom_input.run_if(in_state(AppState::InGame)));
    }
}

fn setup(mut commands: Commands, mut gizmo_config: ResMut<GizmoConfigStore>) {
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

    for (_, config, _) in gizmo_config.iter_mut() {
        config.depth_bias = -1.0;
    }

    commands.spawn((
        WorldCamera,
        InputManagerBundle::with_map(pan_input_map),
        InputManagerBundle::with_map(zoom_input_map),
        Save,
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        Msaa::Off,
    ));
}

impl BuildView for WorldCamera {
    fn build(_world: &World, _object: Object<Self>, mut view: ViewCommands<Self>) {
        view.insert((
            Camera3d::default(),
            Camera {
                order: 0,
                clear_color: ClearColorConfig::None,
                ..Default::default()
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
            RayCastPickable,
        ))
        .insert(DepthPrepass)
        .insert(NormalPrepass)
        .insert(DeferredPrepass)
        .insert(ClusterConfig::FixedZ {
            // 4096 clusters is the Bevy default
            // if you don't have many lights, you can reduce this value
            total: 4096,
            // Bevy default is 24 Z-slices
            // For a top-down-view game, 1 is probably optimal.
            z_slices: 1,
            dynamic_resizing: true,
            z_config: Default::default(),
        });
    }
}

fn handle_pan_input(
    mut query: Query<
        (
            &ActionState<CameraPanAction>,
            &mut AccumulatedInput,
            &mut Acceleration,
        ),
        With<WorldCamera>,
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
    mut query: Query<(&ActionState<CameraZoomAction>)>,
    mut actual_camera: Query<&mut Projection, With<Camera3d>>,
) {
    let zoom_amount = 0.3;
    for (action_state) in &mut query {
        if let Ok(camera_projection) = actual_camera.get_single_mut() {
            if let Projection::Orthographic(ref mut ortho_projection) =
                *camera_projection.into_inner()
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
}
