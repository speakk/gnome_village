use crate::features::position::{PreviousWorldPosition, WorldPosition};
use crate::features::states::AppState;
use bevy::app::RunFixedMainLoopSystem::BeforeFixedMainLoop;
use bevy::prelude::ops::powf;
use bevy::prelude::*;

pub struct MovementPlugin;

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Force(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct InverseMass(f32);

impl InverseMass {
    pub fn new(mass: f32) -> Self {
        Self(1.0 / mass)
    }
}
//
// #[derive(Component, Default)]
// #[derive(Debug)]
// pub struct Acceleration(pub Vec2);

/// Range between 0.0 to 0.3 is sensible
#[derive(Component, Default)]
pub struct Friction(pub f32);

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            set_previous_world_position.in_set(BeforeFixedMainLoop),
        );
        // app.add_systems(
        //     FixedUpdate,
        //     (
        //         apply_acceleration,
        //         apply_velocity,
        //         reset_input,
        //         reset_acceleration,
        //         apply_friction,
        //         clear_force
        //     )
        //         .chain()
        //         .run_if(in_state(AppState::InGame)),
        // );
        app.add_systems(
            FixedUpdate,
            (integrate_acceleration, integrate_velocity, clear_force)
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn set_previous_world_position(mut query: Query<(&WorldPosition, &mut PreviousWorldPosition)>) {
    for (world_position, mut previous_position) in query.iter_mut() {
        previous_position.0 = world_position.0;
    }
}

fn clear_force(mut query: Query<&mut Force>) {
    for mut force in &mut query {
        force.0 = Vec2::ZERO;
    }
}

fn integrate_acceleration(
    mut query: Query<(&mut Velocity, &Force, &InverseMass)>,
    time: Res<Time<Fixed>>,
) {
    for (mut velocity, force, inverse_mass) in &mut query {
        let acceleration = force.0 * inverse_mass.0;
        velocity.0 += acceleration * time.delta_secs();
    }
}

fn integrate_velocity(
    mut query: Query<(&mut WorldPosition, &mut Velocity, &Friction)>,
    time: Res<Time<Fixed>>,
    virtual_time: Res<Time<Virtual>>,
) {
    let clamping_factor = 1.0 - 0.95;
    for (mut world_position, mut velocity, friction) in &mut query {
        world_position.0 += velocity.0 * time.delta_secs();
        // TODO: This virtual time division is my HUNCH because otherwise virtual time speed had no effect
        velocity.0 *= powf(clamping_factor - friction.0, time.delta_secs() / virtual_time.relative_speed());
    }
}

// fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration), With<WorldCamera>>, time: Res<Time<Fixed>>) {
//     for (mut velocity, acceleration) in &mut query {
//         velocity.0 += acceleration.0 * time.delta_secs();
//         println!("Applied acceleration of {:?}, with velocity now being: {:?}", acceleration, velocity);
//     }
// }
//
// fn apply_velocity(
//     mut query: Query<(&mut WorldPosition, &Velocity)>,
//     time: Res<Time<Fixed>>,
// ) {
//     for (mut world_position, velocity) in &mut query {
//         world_position.x += velocity.0.x * time.delta_secs();
//         world_position.y += velocity.0.y * time.delta_secs();
//     }
// }
//
// fn reset_input(mut query: Query<&mut AccumulatedInput>) {
//     for mut input in &mut query {
//         input.0 = Vec2::ZERO;
//     }
// }
//
// fn reset_acceleration(mut query: Query<&mut Acceleration>) {
//     for mut acceleration in &mut query {
//         acceleration.0 = Vec2::ZERO;
//     }
// }
//
// fn apply_friction(mut query: Query<(&Friction, &mut Velocity)>, time: Res<Time<Fixed>>) {
//     for (friction, mut velocity) in &mut query {
//         let friction_direction = -velocity.0.normalize_or_zero();
//         let friction_magnitude = velocity.0.length() * velocity.0.length();
//         let friction_vector = friction_direction * friction_magnitude;
//         let friction_coefficient = friction.0;
//
//         // TODO: This is to stop camera from gliding for too long - this is SILLY
//         const MIN_VELOCITY: f32 = 2.0;
//
//         velocity.0 += friction_coefficient * friction_vector * time.delta_secs();
//         if velocity.0.length() < MIN_VELOCITY {
//             velocity.0 = Vec2::ZERO;
//         }
//     }
// }
