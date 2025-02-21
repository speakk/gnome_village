use crate::features::ai::utility_ai::settler::attach_to_settler;
use crate::features::states::AppState;
use bevy::prelude::*;

pub struct UtilityAiPlugin;

impl Plugin for UtilityAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (attach_to_settler,).run_if(in_state(AppState::InGame)),
        );
    }
}
