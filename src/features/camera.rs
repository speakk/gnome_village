use crate::features::movement::MovementIntent;
use crate::features::input::{CameraPanAction, CameraZoomAction, InGameInputContext};
use crate::features::position::InterpolatePosition;
use crate::features::position::WorldPosition;
use crate::features::states::AppState;
use bevy::app::RunFixedMainLoopSystem::BeforeFixedMainLoop;
use bevy::app::{App, Plugin, RunFixedMainLoop};
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::prepass::{
    DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass,
};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::ecs::prelude::*;
use bevy::math::{Vec2, Vec3};
use bevy::pbr::{Atmosphere, ClusterConfig};
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, Exposure, ScalingMode};
use bevy::render::render_resource::ShaderType;
use bevy::render::view::{Layer, NoIndirectDrawing, RenderLayers};
use bevy_enhanced_input::input::Input::MouseWheel;
use bevy_enhanced_input::prelude::*;
use bevy_hanabi::UnaryOperator::Exp;
use moonshine_core::save::Save;
use moonshine_object::Object;
use moonshine_view::{BuildView, RegisterView, ViewCommands};
use std::ops::{Add, Sub};

pub struct CameraPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(WorldPosition,
    MovementIntent, InterpolatePosition = InterpolatePosition(Some(5.0)))
]
pub struct WorldCamera;

#[derive(InputContext, Reflect)]
pub struct CameraInputContext;


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(AtmospherePlugin)
            .add_input_context::<CameraInputContext>()
            .add_viewable::<WorldCamera>()
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, attach_camera_input_context)
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
    actions.bind::<CameraZoomAction>().to(MouseWheel {
        mod_keys: Default::default(),
    });
}

fn attach_camera_input_context(query: Query<Entity, Added<WorldCamera>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).insert(Actions::<CameraInputContext>::default());
    }
}

fn setup(mut commands: Commands, mut gizmo_config: ResMut<GizmoConfigStore>) {
    for (_, config, _) in gizmo_config.iter_mut() {
        config.depth_bias = -1.0;
        config.render_layers = RenderLayers::layer(1);
    }

    commands.spawn((WorldCamera, Save));

    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            order: 1,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        Msaa::Off,
        // This seems to fix 3d gizmos appearing as small mini versions in the middle of the screen
        RenderLayers::layer(0),
    ));
}

impl BuildView for WorldCamera {
    fn build(_world: &World, _object: Object<Self>, mut view: ViewCommands<Self>) {
        view.insert((
            Camera3d::default(),
            //NoIndirectDrawing,
            Camera {
                order: 0,
                //clear_color: ClearColorConfig::None,
                hdr: true,
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
            RenderLayers::from_layers(&[0, 1]),
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
            Fxaa::default(),
            Atmosphere::EARTH,
            Exposure { ev100: 14.2 },
            Tonemapping::AcesFitted,
            Bloom::NATURAL,
        ))
        .insert(DepthPrepass)
        .insert(NormalPrepass)
        .insert(MotionVectorPrepass)
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
    actual_camera: Query<&Projection, With<Camera3d>>,
    mut query: Query<&mut MovementIntent, With<WorldCamera>>,
) {
    let force_multiplier = 20.0;
    if let Ok(Projection::Orthographic(ortho_projection)) = actual_camera.single() {
        {
            //ortho_projection.scale

            let scale_multiplier = (ortho_projection.scale + 1.0) / 1.0;
            println!("scale_multiplier: {}", scale_multiplier);

            for mut input in &mut query {
                let mut value = trigger.value;
                println!("value: {:?}", value);
                value.y *= -1.0;
                input.0 = value.normalize_or_zero() * force_multiplier * scale_multiplier;
            }
        }
    }
}

fn handle_zoom_input(
    trigger: Trigger<Fired<CameraZoomAction>>,
    mut actual_camera: Query<&mut Projection, With<Camera3d>>,
) {
    let zoom_amount = 0.3;
    if let Ok(camera_projection) = actual_camera.get_single_mut() {
        let value = trigger.value.y;

        if let Projection::Orthographic(ref mut ortho_projection) = *camera_projection.into_inner()
        {
            ortho_projection.scale = ortho_projection
                .scale
                .add(value * zoom_amount)
                .clamp(0.4, 4.0);
        }
    }
}
