use crate::features::input::CameraPanAction;
use crate::features::movement::{Acceleration, Friction, Velocity};
use crate::features::position::{PreviousWorldPosition, WorldPosition};
use bevy::app::RunFixedMainLoopSystem::BeforeFixedMainLoop;
use bevy::app::{App, Plugin, RunFixedMainLoop, Startup};
use bevy::asset::LoadState;
use bevy::core_pipeline::Skybox;
use bevy::ecs::prelude::*;
use bevy::image::{ImageSampler, ImageSamplerDescriptor};
use bevy::math::{Vec2, Vec3};
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy_atmosphere::plugin::{AtmosphereCamera, AtmospherePlugin};
use leafwing_input_manager::prelude::*;

pub struct CameraPlugin;

#[derive(Resource)]
pub struct SkyBoxMap {
    pub image: Handle<Image>,
    pub loaded: bool,
}
/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(pub Vec2);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_systems(Startup, setup)
            .add_systems(RunFixedMainLoop, handle_input.in_set(BeforeFixedMainLoop))
            .add_systems(Update, reinterpret_cubemap);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let input_map = InputMap::new([
        (CameraPanAction::Left, KeyA),
        (CameraPanAction::Right, KeyD),
        (CameraPanAction::Up, KeyW),
        (CameraPanAction::Down, KeyS),
    ]);

    let sky_image_handle = asset_server.load::<Image>("skybox.png");
    commands.insert_resource(SkyBoxMap {
        image: sky_image_handle.clone(),
        loaded: false,
    });

    // camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 48.0,
            },
            near: 0.1,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        ScreenSpaceAmbientOcclusion::default(),
        Msaa::Off,
        AtmosphereCamera::default(),
        // Skybox {
        //     image: sky_image_handle.clone(),
        //     brightness: 6000.0,
        //     rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2 / 1.5),
        // },
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

pub fn reinterpret_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<SkyBoxMap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.loaded && matches!(asset_server.load_state(&cubemap.image), LoadState::Loaded) {
        cubemap.loaded = true;
        let image = images.get_mut(&cubemap.image).unwrap();

        if image.texture_descriptor.array_layer_count() == 1 {
            //6
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..Default::default()
            });
        }
        //set all skybox images to the new array texture
        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image.clone();
        }
    }
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
