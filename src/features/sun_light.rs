use bevy::app::{App, Startup};
use bevy::color::palettes::css::ORANGE_RED;
use bevy::color::palettes::tailwind::SKY_200;
use bevy::pbr::{AmbientLight, CascadeShadowConfigBuilder, DirectionalLight};
use bevy::prelude::*;

pub struct SunLightPlugin;

impl Plugin for SunLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lights);
    }
}

pub fn setup_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 10.0,
    });

    commands.spawn((
        DirectionalLight {
            color: Color::from(SKY_200),
            illuminance: 900.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.9, 0.9, 0.8),
            illuminance: 6000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-15.0, 10.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 100.0,
            ..default()
        }
        .build(),
    ));
}
