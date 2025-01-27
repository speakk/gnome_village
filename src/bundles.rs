use crate::bundles::buildables::BuildablesPlugin;
use crate::bundles::settler::Settler;
use crate::features::misc_components::Prototype;
use bevy::prelude::*;
use moonshine_core::save::Save;
use moonshine_view::RegisterView;

pub mod buildables;
pub mod rock;
pub mod settler;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuildablesPlugin).add_viewable::<Settler>();
    }
}

pub fn make_concrete_from_prototype(prototype: Entity, mut commands: Commands) -> Entity {
    let cloned = prototype;
    commands
        .entity(cloned)
        .insert(Save)
        .remove::<Prototype>()
        .id()
}
