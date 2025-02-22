use crate::bundles::spawners::setup_spawners_and_prototypes;
use crate::features::states::AppState;
use bevy::app::App;
use bevy::prelude::{NextState, OnEnter, Plugin, ResMut};

pub struct PreloadPlugin;

impl Plugin for PreloadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Preload),
            (
                setup_spawners_and_prototypes,
                |mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::MapGeneration);
                },
            ),
        );
    }
}
