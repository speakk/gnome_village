use crate::features::input::{CameraPanAction, CameraZoomAction, InGameInputContext};
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
use bevy_enhanced_input::prelude::*;
use bevy::render::camera::{CameraOutputMode, ScalingMode};
use moonshine_core::save::Save;
use moonshine_object::Object;
use moonshine_view::{BuildView, RegisterView, ViewCommands};
use std::ops::{Add, Sub};
use bevy::render::view::RenderLayers;
use bevy_enhanced_input::input::Input::MouseWheel;

pub struct CameraPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(WorldPosition, Velocity, Acceleration, Friction = Friction(2.0), AccumulatedInput, InterpolatePosition)]
pub struct WorldCamera;

#[derive(InputContext, Reflect)]
pub struct CameraInputContext;

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(pub Vec2);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(AtmospherePlugin)
            .add_input_context::<CameraInputContext>()
            .add_viewable::<WorldCamera>()
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_observer(binding)
            .add_observer(handle_pan_input)
            .add_observer(handle_zoom_input);
    }
}

fn binding(
    trigger: Trigger<Binding<CameraInputContext>>,
    mut in_game_input_context: Query<&mut Actions<CameraInputContext>>,
) {
    let mut actions = in_game_input_context.get_mut(trigger.target()).unwrap();

    actions.bind::<CameraPanAction>().to(Cardinal::wasd_keys());
    actions.bind::<CameraZoomAction>().to(MouseWheel { mod_keys: Default::default() });
}

fn setup(mut commands: Commands, mut gizmo_config: ResMut<GizmoConfigStore>) {
    for (_, config, _) in gizmo_config.iter_mut() {
        config.depth_bias = -1.0;
    }

    commands.spawn((
        WorldCamera,
        Actions::<CameraInputContext>::default(),
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
        // This seems to fix 3d gizmos appearing as small mini versions in the middle of the screen
        RenderLayers::layer(1)
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
            //AtmosphereCamera::default(),
            // Skybox {
            //     image: sky_image_handle.clone(),
            //     brightness: 6000.0,
            //     rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2 / 1.5),
            // },
            Transform::from_xyz(0.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            MeshPickingCamera,
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
    trigger: Trigger<Fired<CameraPanAction>>,
    mut query: Query<&mut Acceleration, With<WorldCamera>>,
) {
    let accel_speed = 673.0;
    for mut acceleration in &mut query {
        let mut value = trigger.value;
        value.y *= -1.0;
        *acceleration = Acceleration(value.normalize_or_zero() * accel_speed);
    }
}

fn handle_zoom_input(
    trigger: Trigger<Fired<CameraZoomAction>>,
    mut actual_camera: Query<&mut Projection, With<Camera3d>>,
) {
    let zoom_amount = 0.3;
    if let Ok(camera_projection) = actual_camera.get_single_mut() {
        let value = trigger.value.y;

        if let Projection::Orthographic(ref mut ortho_projection) =
            *camera_projection.into_inner()
        {
            ortho_projection.scale = ortho_projection.scale.add(value * zoom_amount).clamp(0.4, 3.0);
        }
    }
}
