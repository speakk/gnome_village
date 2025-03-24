use crate::features::states::AppState;
use bevy::app::App;
use bevy::color::palettes::css::ORANGE_RED;
use bevy::color::palettes::tailwind::SKY_200;
use bevy::pbr::light_consts::lux::AMBIENT_DAYLIGHT;
use bevy::pbr::{AmbientLight, CascadeShadowConfigBuilder, DirectionalLight};
use bevy::prelude::*;
use bevy_atmosphere::model::AtmosphereModel;
use bevy_atmosphere::prelude::{AtmosphereMut, Nishita};
use std::f32::consts::PI;
use bevy::color::palettes::basic::NAVY;
use bevy::window::WindowResized;

pub struct SunLightPlugin;

// Marker for updating the position of the light, not needed unless we have multiple lights
#[derive(Component)]
struct Sun;

#[derive(Component)]
struct Moon;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlanetOrigin;

// Timer for updating the daylight cycle
#[derive(Resource)]
struct CycleTimer(Timer);

#[derive(Resource)]
struct AtmosphereTimer(Timer);

#[derive(Event)]
pub struct DayTimeChanged(pub f32);

impl Plugin for SunLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_lights)
            .insert_resource(CycleTimer(Timer::new(
                bevy::utils::Duration::from_millis(50),
                TimerMode::Repeating,
            )))
            .insert_resource(AtmosphereTimer(Timer::new(
                bevy::utils::Duration::from_millis(2000),
                TimerMode::Repeating,
            )))
            .insert_resource(AtmosphereModel::default())
            .add_event::<DayTimeChanged>()
            .add_systems(Update, daylight_cycle.run_if(in_state(AppState::InGame)));

    }
}

pub fn setup_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 10.0,
    });

    commands
        .spawn((PlanetOrigin, Transform::default(), Name::from("PlanetOrigin")))
        .with_children(move |child_builder| {
            let maximum_shadow_distance = 100.0;
            let cascade_count = 1;

            child_builder.spawn((
                Sun,
                DirectionalLight {
                    color: Color::srgb(0.9, 0.9, 0.8),
                    illuminance: 6000.0,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                CascadeShadowConfigBuilder {
                    num_cascades: cascade_count,
                    first_cascade_far_bound: 4.0,
                    maximum_distance: maximum_shadow_distance,
                    ..default()
                }
                .build(),
            ));

            child_builder.spawn((
                Moon,
                DirectionalLight {
                    color: Color::srgb(0.4, 0.4, 1.0),
                    illuminance: 4000.0,
                    shadows_enabled: false,
                    ..default()
                },
                Visibility::Hidden,
                Transform::from_xyz(0.0, -10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                // The default cascade config is designed to handle large scenes.
                // As this example has a much smaller world, we can tighten the shadow
                // bounds for better visual quality.
                CascadeShadowConfigBuilder {
                    num_cascades: cascade_count,
                    first_cascade_far_bound: 4.0,
                    minimum_distance: 0.0,
                    maximum_distance: maximum_shadow_distance,
                    ..default()
                }
                .build(),
            ));
        });

    // Kind of ambient light
    commands.spawn((
        DirectionalLight {
            color: Color::from(SKY_200),
            illuminance: 900.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// We can edit the Atmosphere resource and it will be updated automatically
#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn daylight_cycle(
    mut commands: Commands,
    mut atmosphere: AtmosphereMut<Nishita>,
    mut sun_query: Query<
        (&mut Transform, &mut DirectionalLight, Entity),
        (With<Sun>, Without<Moon>),
    >,
    mut moon_query: Query<
        (&mut Transform, &mut DirectionalLight, Entity),
        (With<Moon>, Without<Sun>),
    >,
    mut planet_origin: Query<&mut Transform, (Without<Moon>, Without<Sun>, With<PlanetOrigin>)>,
    mut visibility_query: Query<&mut Visibility>,
    mut timer: ResMut<CycleTimer>,
    mut atmosphere_timer: ResMut<AtmosphereTimer>,
    mut day_time_changed: EventWriter<DayTimeChanged>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    atmosphere_timer.0.tick(time.delta());

    // TODO: Figure out the math in this, this was partially straight from bevy_atmosphere example

    let timer_scale_division = 6.0;
    let t = time.elapsed_secs_wrapped() / timer_scale_division;

    if atmosphere_timer.0.finished() {
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());
    }

    if timer.0.finished() {
        day_time_changed.send(DayTimeChanged(t));
        
        if let Ok(mut transform) = planet_origin.get_single_mut() {
            // transform.rotation = Quat::from_rotation_z(PI / 2.0);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, t, t, 0.0);
            //transform.rotate_local_x(t)
        }

        if let Some((mut light_trans, mut directional, entity)) = sun_query.single_mut().into() {
            light_trans.look_at(Vec3::ZERO, Vec3::Y);
            //light_trans.rotation = Quat::from_rotation_x(-t);
            // let illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
            // // TODO: Base this on rotation
            // if illuminance < 10.0 {
            //     if visibility_query.get_mut(entity).is_ok() {
            //         commands.entity(entity).remove::<Visibility>();
            //     }
            // } else if let Err(_visibility) = visibility_query.get_mut(entity) {
            //     commands.entity(entity).insert(Visibility::Visible);
            // }
            // directional.illuminance = illuminance;1
        }

        if let Some((mut light_trans, mut directional, entity)) = moon_query.single_mut().into() {
            // let moon_t = -t - PI;
            // //light_trans.rotation = Quat::from_rotation_x(moon_t);
            // let illuminance = (-moon_t).sin().max(0.2).powf(2.0) * AMBIENT_DAYLIGHT * 0.5;
            //
            // // TODO: Base this on rotation
            // if illuminance < 201.0 {
            //     if visibility_query.get_mut(entity).is_ok() {
            //         commands.entity(entity).remove::<Visibility>();
            //     }
            // } else if let Err(_visibility) = visibility_query.get_mut(entity) {
            //     commands.entity(entity).insert(Visibility::Visible);
            // }
            //
            // directional.illuminance = illuminance;
        }
    }
}
