use crate::features::states::AppState;
use bevy::app::App;
use bevy::color::palettes::basic::NAVY;
use bevy::color::palettes::css::ORANGE_RED;
use bevy::color::palettes::tailwind::SKY_200;
use bevy::pbr::light_consts::lux::AMBIENT_DAYLIGHT;
use bevy::pbr::{AmbientLight, CascadeShadowConfigBuilder, DirectionalLight};
use bevy::prelude::*;
use bevy::window::WindowResized;
use std::f32::consts::{PI, TAU};

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
pub struct CurrentTimeOfDay {
    pub total_time: f32,
    /// 0.0 midnight, 0.5 noon
    pub time_of_day: f32,
}

impl Default for CurrentTimeOfDay {
    fn default() -> Self {
        CurrentTimeOfDay {
            total_time: 0.25, // Let's start in the morning
            time_of_day: 0.0,
        }
    }
}

impl Plugin for SunLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_lights)
            .insert_resource(CycleTimer(Timer::new(
                bevy::utils::Duration::from_millis(50),
                TimerMode::Repeating,
            )))
            .insert_resource(AtmosphereTimer(Timer::new(
                bevy::utils::Duration::from_millis(50),
                TimerMode::Repeating,
            )))
            .insert_resource(CurrentTimeOfDay::default())
            .add_systems(
                Update,
                (daylight_cycle, rotate_planet,
                    //rotate_atmosphere
            )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

pub fn setup_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 10.0,
    });

    commands
        .spawn((
            PlanetOrigin,
            Transform::default(),
            Name::from("PlanetOrigin"),
        ))
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
                    shadows_enabled: true,
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
fn daylight_cycle(mut current_time_of_day: ResMut<CurrentTimeOfDay>, time: Res<Time>) {
    // At 60.0 one day takes a minute
    let timer_scale_division = 60.0;
    let total_time = current_time_of_day.total_time + time.delta_secs() / timer_scale_division;

    let time_of_day = total_time % 1.0;
    current_time_of_day.total_time = total_time;
    current_time_of_day.time_of_day = time_of_day;
}

#[derive(Resource)]
struct AtmosphereTimer(Timer);

fn rotate_planet(
    mut sun_query: Query<
        (&mut Transform, &mut DirectionalLight, &mut Visibility),
        (With<Sun>, Without<Moon>),
    >,
    mut moon_query: Query<
        (&mut Transform, &mut DirectionalLight, &mut Visibility),
        (With<Moon>, Without<Sun>),
    >,
    mut planet_origin: Query<&mut Transform, (Without<Moon>, Without<Sun>, With<PlanetOrigin>)>,
    current_time_of_day: Res<CurrentTimeOfDay>,
) {
    if !current_time_of_day.is_changed() {
        return;
    }

    let t = current_time_of_day.time_of_day;

    if let Ok(mut transform) = planet_origin.get_single_mut() {
        transform.rotation = Quat::from_euler(EulerRot::YXZ, t * PI / 4.0, t * TAU + PI, 0.0);
    }

    if let Ok((mut sun_transform, mut sun_light, mut sun_visibility)) = sun_query.get_single_mut() {
        if let Some((mut moon_transform, mut moon_light, mut moon_visibility)) =
            moon_query.single_mut().into()
        {
            if (0.25..=0.75).contains(&t) {
                *sun_visibility = Visibility::Visible;
                *moon_visibility = Visibility::Hidden;

                sun_transform.look_at(Vec3::ZERO, Vec3::Y);
                let illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
                sun_light.illuminance = illuminance;
            } else {
                *moon_visibility = Visibility::Visible;
                *sun_visibility = Visibility::Hidden;

                moon_transform.look_at(Vec3::ZERO, Vec3::Y);
                //let illuminance = (t + 0.75).sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
                //moon_light.illuminance = illuminance;
            }
        }
    }
}
// 
// fn rotate_atmosphere(
//     mut atmosphere: AtmosphereMut<Nishita>,
//     mut atmosphere_timer: ResMut<AtmosphereTimer>,
//     current_time_of_day: Res<CurrentTimeOfDay>,
//     time: Res<Time>,
// ) {
//     atmosphere_timer.0.tick(time.delta());
//     if atmosphere_timer.0.finished() {
//         let t = current_time_of_day.time_of_day;
//         atmosphere.sun_position =
//             Quat::from_euler(EulerRot::YXZ, t * PI / 4.0, t * PI * 2.0 + PI, 0.0).xyz()
//                 * Vec3::new(1.0, -1.0, 1.0);
//     }
// }
