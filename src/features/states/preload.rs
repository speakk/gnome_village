use bevy::app::App;
use bevy::prelude::{NextState, OnEnter, Plugin, ResMut};
use crate::bundles::spawners::setup_spawners;
use crate::features::states::AppState;

pub struct PreloadPlugin;

impl Plugin for PreloadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Preload),
            (
                setup_spawners,
                |mut next_state: ResMut<NextState<AppState>>| { next_state.set(AppState::MapGeneration);})
        );
    }
}