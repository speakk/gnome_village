use crate::features::world_interaction::mouse_selection::MouseSelectionPlugin;
use bevy::prelude::*;

pub mod mouse_selection;
pub mod build_action;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MouseSelectionPlugin);
    }
}
