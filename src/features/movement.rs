use crate::features::position::{PreviousWorldPosition, WorldPosition};
use crate::features::states::AppState;
use bevy::prelude::*;

pub struct MovementPlugin;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct MovementIntent(pub Vec2);


// TODO: Transform interpolation should happen after apply_movement, but before clear_input.
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement.run_if(in_state(AppState::InGame)));
        app.add_systems(PostUpdate, clear_input.run_if(in_state(AppState::InGame)));
    }
}

fn clear_input(mut query: Query<&mut MovementIntent>) {
    for mut input in &mut query {
        input.0 = Vec2::ZERO;
    }
}

fn apply_movement(mut query: Query<(&MovementIntent, &mut WorldPosition)>, time: Res<Time>) {
    for (input, mut world_position) in &mut query {
        world_position.0 += input.0 * time.delta_secs();
    }
}