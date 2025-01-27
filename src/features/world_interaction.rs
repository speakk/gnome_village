use crate::features::world_interaction::build_action::BuildActionPlugin;
use crate::features::world_interaction::mouse_selection::MouseSelectionPlugin;
use bevy::prelude::*;

pub mod build_action;
pub mod mouse_selection;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BuildActionPlugin, MouseSelectionPlugin));
    }
}
