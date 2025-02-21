use crate::features::world_interaction::build_action::BuildActionPlugin;
use crate::features::world_interaction::cancel_job::CancelJobPlugin;
use crate::features::world_interaction::mouse_selection::MouseSelectionPlugin;
use crate::features::world_interaction::world_speed::WorldSpeedPlugin;
use bevy::prelude::*;

pub mod build_action;
pub mod cancel_job;
pub mod mouse_selection;
pub mod world_speed;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BuildActionPlugin,
            MouseSelectionPlugin,
            CancelJobPlugin,
            WorldSpeedPlugin,
        ));
    }
}
