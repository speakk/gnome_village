use bevy::app::{App, Startup};
use bevy::color::palettes::css::ORANGE_RED;
use bevy::pbr::{light_consts, AmbientLight, CascadeShadowConfigBuilder, DirectionalLight};
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct SunLightPlugin;

impl Plugin for SunLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lights);
    }
}

pub fn setup_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 0.02,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 5.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            //first_cascade_far_bound: 4.0,
            //maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
}
