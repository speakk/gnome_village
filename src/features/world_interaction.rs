use bevy::prelude::*;
use crate::features::world_interaction::mouse_selection::MouseSelectionPlugin;

pub mod mouse_selection;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MouseSelectionPlugin);
    }
}